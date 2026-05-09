use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::config::Region;
use std::error::Error;
use std::env;

pub struct Storage {
    client: Client,
    bucket: String,
}

impl Storage {
    pub async fn new() -> Self {
        let endpoint_url = env::var("S3_ENDPOINT").ok();
        let region = env::var("S3_REGION").unwrap_or_else(|_| "us-east-1".to_string());
        let bucket = env::var("S3_BUCKET").expect("S3_BUCKET must be set");

        let mut config_loader = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region(Region::new(region));

        if let Some(url) = endpoint_url {
            config_loader = config_loader.endpoint_url(url);
        }

        let config = config_loader.load().await;
        let client = Client::new(&config);
        
        Storage { client, bucket }
    }

    pub async fn upload_attachment(
        &self,
        message_id: &str,
        filename: &str,
        content: Vec<u8>,
    ) -> Result<String, Box<dyn Error>> {
        let key = format!("attachments/{}/{}", message_id, filename);
        
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .body(ByteStream::from(content))
            .send()
            .await?;

        Ok(key)
    }

    pub async fn upload_raw_mime(
        &self,
        message_id: &str,
        content: Vec<u8>,
    ) -> Result<String, Box<dyn Error>> {
        let key = format!("raw/{}.eml", message_id);
        
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .body(ByteStream::from(content))
            .send()
            .await?;

        Ok(key)
    }
}
