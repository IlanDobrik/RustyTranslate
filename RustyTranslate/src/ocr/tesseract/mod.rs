use std::io::Write;

use super::Ocr;
use anyhow::{Context, Result, anyhow};

pub struct TesseractOcr;

impl Ocr for TesseractOcr {
    fn scan_text(&self, image: &[u8]) -> Result<Vec<String>> {
        let mut tmpfile = tempfile::NamedTempFile::new()?;
        tmpfile.write_all(image).context("failed to write image")?;
        tmpfile.flush()?;

        let output = std::process::Command::new("tesseract")
            .args([
                tmpfile.path().to_str().ok_or(anyhow!("no path"))?,
                "-",
                "-l",
                "ita",
            ])
            .output()
            .context("failed to get output")?;

        let text = String::from_utf8(output.stdout)
            .map_err(|_e| anyhow!("failed to convert output to string"))?
            .split("\n")
            .filter(|line| !line.trim().is_empty())
            .map(str::to_string)
            .collect::<Vec<String>>();

        Ok(text)
    }
}

impl TesseractOcr {
    pub fn new() -> Self {
        Self {}
    }
}
