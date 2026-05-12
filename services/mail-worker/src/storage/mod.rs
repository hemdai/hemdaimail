use std::error::Error;
use std::env;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use std::path::PathBuf;

pub struct Storage {
    base_path: PathBuf,
}

impl Storage {
    pub async fn new() -> Self {
        let path = env::var("STORAGE_PATH").unwrap_or_else(|_| "./data".to_string());
        let base_path = PathBuf::from(path);
        if let Err(e) = fs::create_dir_all(&base_path).await {
             eprintln!("Warning: could not create storage dir: {}", e);
        }
        Storage { base_path }
    }

    pub async fn upload_attachment(
        &self,
        message_id: &str,
        filename: &str,
        content: Vec<u8>,
    ) -> Result<String, Box<dyn Error>> {
        let dir = self.base_path.join(format!("attachments/{}", message_id));
        fs::create_dir_all(&dir).await?;
        let path = dir.join(filename);
        let mut file = fs::File::create(&path).await?;
        file.write_all(&content).await?;
        Ok(path.to_string_lossy().to_string())
    }

    pub async fn upload_raw_mime(
        &self,
        message_id: &str,
        content: Vec<u8>,
    ) -> Result<String, Box<dyn Error>> {
        let dir = self.base_path.join("raw");
        fs::create_dir_all(&dir).await?;
        let path = dir.join(format!("{}.eml", message_id));
        let mut file = fs::File::create(&path).await?;
        file.write_all(&content).await?;
        Ok(path.to_string_lossy().to_string())
    }
}
