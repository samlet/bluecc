use tokio::fs::File;
use tokio::io::{BufReader, AsyncBufReadExt};

pub async fn read_lines(file_path: &str) -> crate::Result<Vec<String>> {
    let f = File::open(file_path).await?;
    let mut lines = BufReader::new(f).lines();

    let mut result = Vec::new();
    while let Some(line) = lines.next_line().await? {
        result.push(line);
    }

    Ok(result)
}

