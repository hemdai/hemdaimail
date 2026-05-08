use async_imap::Session;
use async_native_tls::TlsStream;
use async_std::net::TcpStream;
use std::error::Error;
use async_std::stream::StreamExt;

pub type ImapSession = Session<TlsStream<TcpStream>>;

pub struct ImapClient {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password_encrypted: String,
}

impl ImapClient {
    pub async fn connect(&self) -> Result<ImapSession, Box<dyn Error>> {
        let stream = TcpStream::connect((self.host.as_str(), self.port)).await?;
        let tls_stream = async_native_tls::connect(&self.host, stream).await?;

        let client = async_imap::Client::new(tls_stream);
        
        let session = client.login(&self.username, &self.password_encrypted).await
            .map_err(|(e, _)| e)?;

        Ok(session)
    }
}

pub async fn sync_mailboxes(session: &mut ImapSession) -> Result<Vec<String>, Box<dyn Error>> {
    let mut folders_stream = session.list(None, Some("*")).await?;
    let mut folder_names = Vec::new();
    
    while let Some(folder) = folders_stream.next().await {
        let folder = folder?;
        folder_names.push(folder.name().to_string());
    }
    
    Ok(folder_names)
}
