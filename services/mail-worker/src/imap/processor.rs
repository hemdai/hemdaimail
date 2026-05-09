use mailparse::{parse_mail, ParsedMail, MailHeaderMap};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessedMessage {
    pub message_id: Option<String>,
    pub in_reply_to: Option<String>,
    pub references: Vec<String>,
    pub subject: Option<String>,
    pub from: Option<String>,
    pub to: Vec<String>,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub attachments: Vec<ProcessedAttachment>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessedAttachment {
    pub filename: String,
    pub content_type: String,
    pub size: usize,
    pub content: Vec<u8>,
}

pub fn process_raw_message(raw_mime: &[u8]) -> Result<ProcessedMessage, Box<dyn Error>> {
    let parsed = parse_mail(raw_mime)?;
    let headers = parsed.get_headers();

    let message_id = headers.get_first_value("Message-ID");
    let in_reply_to = headers.get_first_value("In-Reply-To");
    let subject = headers.get_first_value("Subject");
    let from = headers.get_first_value("From");
    
    let to = headers.get_all_values("To")
        .iter()
        .flat_map(|v| v.split(',').map(|s| s.trim().to_string()))
        .collect();

    let references = headers.get_first_value("References")
        .map(|v| v.split_whitespace().map(|s| s.to_string()).collect())
        .unwrap_or_else(Vec::new);

    let mut body_text = None;
    let mut body_html = None;
    let mut attachments = Vec::new();

    extract_parts(&parsed, &mut body_text, &mut body_html, &mut attachments)?;

    // Sanitize HTML
    let sanitized_html = body_html.map(|html| ammonia::clean(&html));

    Ok(ProcessedMessage {
        message_id,
        in_reply_to,
        references,
        subject,
        from,
        to,
        body_text,
        body_html: sanitized_html,
        attachments,
    })
}

fn extract_parts(
    mail: &ParsedMail,
    body_text: &mut Option<String>,
    body_html: &mut Option<String>,
    attachments: &mut Vec<ProcessedAttachment>,
) -> Result<(), Box<dyn Error>> {
    let content_type = mail.get_headers().get_first_value("Content-Type").unwrap_or_default();

    if content_type.contains("text/plain") && body_text.is_none() {
        *body_text = Some(mail.get_body()?);
    } else if content_type.contains("text/html") && body_html.is_none() {
        *body_html = Some(mail.get_body()?);
    } else if mail.get_content_disposition().disposition == mailparse::DispositionType::Attachment {
        let filename = mail.get_content_disposition().params.get("filename")
            .cloned()
            .unwrap_or_else(|| "unnamed".to_string());
        
        attachments.push(ProcessedAttachment {
            filename,
            content_type,
            size: mail.get_body_raw()?.len(),
            content: mail.get_body_raw()?,
        });
    }

    for part in &mail.subparts {
        extract_parts(part, body_text, body_html, attachments)?;
    }

    Ok(())
}
