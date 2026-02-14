pub mod tesseract;

use anyhow::Result;

pub trait Ocr {
    fn scan_text(&self, image: &[u8]) -> Result<Vec<String>>;
}
