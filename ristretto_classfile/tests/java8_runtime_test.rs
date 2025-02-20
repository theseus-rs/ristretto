mod utilities;

use anyhow::{Result, anyhow};
use flate2::read::GzDecoder;
use reqwest::Client;
use std::io::Read;
use tar::Archive;

#[tokio::test]
async fn verify() -> Result<()> {
    let url = "https://corretto.aws/downloads/latest/amazon-corretto-8-x64-linux-jdk.tar.gz";
    let client = Client::new();
    let archive = client.get(url).send().await?.bytes().await?.to_vec();
    let jar_bytes = get_runtime_jar(archive)?;

    utilities::jar::verify(jar_bytes)?;

    Ok(())
}

fn get_runtime_jar(archive: Vec<u8>) -> Result<Vec<u8>> {
    let tar = GzDecoder::new(&*archive);
    let mut archive = Archive::new(tar);
    let file_name = "rt.jar";
    let mut runtime_jar: Result<Vec<u8>> = Err(anyhow!("{file_name} not found in the archive"));

    for file in archive.entries()? {
        let mut file = file?;
        if file.path()?.ends_with(file_name) {
            let mut jar_bytes = Vec::new();
            file.read_to_end(&mut jar_bytes)?;
            runtime_jar = Ok(jar_bytes);
            break;
        }
    }

    runtime_jar
}
