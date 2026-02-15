pub mod basic;

use anyhow::Result;

pub trait Translator {
    fn translate(&self, text: &str) -> Result<String>;
}
