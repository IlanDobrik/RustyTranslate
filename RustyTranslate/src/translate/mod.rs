use anyhow::Result;

pub trait Translator {
    fn translate(&self, text: &str) -> Result<String>;
}

pub struct BasicTranslator;

impl Translator for BasicTranslator {
    fn translate(&self, text: &str) -> Result<String> {
        let url = format!(
            "https://translate.googleapis.com/translate_a/single?client=gtx&sl={}&tl={}&dt=t&q={}",
            "it", "en", text
        );

        let response = reqwest::blocking::get(&url)?.text()?;
        let translated_text: String = serde_json::from_str::<serde_json::Value>(&response)?[0][0]
            [0]
        .as_str()
        .unwrap()
        .to_string();

        Ok(translated_text)
    }
}

impl BasicTranslator {
    pub fn new() -> Self {
        Self {}
    }
}
