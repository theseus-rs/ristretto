mod utilities;

use flate2::read::GzDecoder;
use reqwest::Client;
use std::error::Error;
use std::io::Read;
use tar::Archive;

#[tokio::test]
async fn verify() -> Result<(), Box<dyn Error>> {
    let url = "https://corretto.aws/downloads/latest/amazon-corretto-8-x64-linux-jdk.tar.gz";
    let client = Client::new();
    let archive = client.get(url).send().await?.bytes().await?.to_vec();
    let jar_bytes = get_runtime_jar(archive)?;

    utilities::jar::verify(jar_bytes)?;

    Ok(())
}

fn get_runtime_jar(archive: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    let tar = GzDecoder::new(&*archive);
    let mut archive = Archive::new(tar);

    for file in archive.entries()? {
        let mut file = file?;
        if file.path()?.ends_with("rt.jar") {
            let mut jar_bytes = Vec::new();
            file.read_to_end(&mut jar_bytes)?;
            return Ok(jar_bytes);
        }
    }

    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "rt.jar not found in the archive",
    )))
}
