use anyhow::{Result, Context};
use std::fs::File;
use tempfile::Builder;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("HTTP request error")]
    HttpRequest(#[from] reqwest::Error),
}

#[tokio::main]
async fn main() -> Result<()> {
    let tmp_dir  = Builder::new().prefix("logo").tempdir()?;
    let target = "https://www.rust-lang.org/static/images/rust-logo-blk.png";
    let response = reqwest::get(target).await?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty(){None} else {Some(name)})
            .unwrap_or("tmp.bin");

    println!("file to download: '{}'", fname);
    let fname = tmp_dir.path().join(fname);
    println!("will be located under: '{:?}'", fname);
        File::create(fname)?

    };
    let content = response.bytes().await?;
    std::io::copy(&mut &content[..], &mut dest).context("Failed to write to file ")?;

    Ok(())
}