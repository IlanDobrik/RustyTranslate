use super::Capturer;
use anyhow::{Result, anyhow};
use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

pub struct BasicCapturer {
    files: Vec<String>,
}

impl Capturer for BasicCapturer {
    fn capture(&mut self) -> Result<Vec<u8>> {
        let new_files = scan()?
            .into_iter()
            .filter(|entry| !self.files.contains(entry))
            .collect::<Vec<String>>();
        let Some(new_file) = new_files.first() else {
            return Err(anyhow!("no new files"));
        };

        self.files.push(new_file.to_owned());

        let mut file = File::open(new_file)?;

        let mut buf: Vec<u8> = vec![];
        file.read_to_end(&mut buf)?;

        Ok(buf)
    }
}

impl BasicCapturer {
    pub fn new() -> Result<Self> {
        let files = scan()?;

        Ok(Self { files })
    }
}

fn scan() -> Result<Vec<String>> {
    let userprofile = std::env::var("USERPROFILE")?;
    let path = format!(r"{userprofile}\OneDrive\Pictures\Screenshots 1");
    let path = Path::new(&path);
    Ok(fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.path().to_str().map(|s| s.to_string()))
        .collect())
}
