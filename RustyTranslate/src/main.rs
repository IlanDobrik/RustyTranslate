mod capture;
mod display;
mod ocr;
mod pipeline;
mod translate;

use anyhow::Result;

use crate::capture::Capturer;
use crate::capture::basic::BasicCapturer;
use crate::display::Displayer;
use crate::display::notification::Notification;
use crate::ocr::Ocr;
use crate::ocr::tesseract::TesseractOcr;
use crate::pipeline::{Pipeline, SimplePipe};
use crate::translate::{BasicTranslator, Translator};

fn res_main() -> Result<()> {
    let mut pipe = SimplePipe::new(1);

    let mut capturer = BasicCapturer::new()?;
    let aocr = TesseractOcr::new();
    let translator = BasicTranslator::new();
    let displayer = Notification::new();

    pipe.register_func(Box::new(move |_| {
        let file = capturer.capture()?;
        Ok(file)
    }));

    pipe.register_func(Box::new(move |image: &[u8]| {
        let texts = aocr.scan_text(image)?;
        println!("texts: {texts:?}");
        Ok(texts.join("\n").into_bytes())
    }));

    pipe.register_func(Box::new(move |text_bytes: &[u8]| {
        let text = str::from_utf8(text_bytes)?;
        let translated = translator.translate(text)?;
        println!("translated: {translated:?}\n");

        Ok(translated.into_bytes())
    }));

    pipe.register_func(Box::new(move |text_bytes: &[u8]| {
        let text = str::from_utf8(text_bytes)?;
        displayer.display(text)?;

        Ok(vec![])
    }));

    println!("ready");
    pipe.run()?;

    Ok(())
}

fn main() {
    match res_main() {
        Ok(_) => println!("finished successfully"),
        Err(e) => println!("err: {e}"),
    }
}
