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

use crate::translate::Translator;
use crate::translate::basic::BasicTranslator;

use crate::pipeline::{Pipeline, SimplePipe, State};

fn res_main() -> Result<()> {
    let mut pipe = SimplePipe::new(1);

    let mut capturer = BasicCapturer::new()?;
    let aocr = TesseractOcr::new();
    let translator = BasicTranslator::new("it".to_string(), "en".to_string());
    let displayer = Notification::new();

    pipe.register_func(Box::new(move |state: &mut State| {
        let file = capturer.capture()?;
        state.image = file;
        Ok(state)
    }));

    pipe.register_func(Box::new(move |state: &mut State| {
        let texts = aocr.scan_text(&state.image)?;
        println!("texts: {texts:?}");
        state.texts = texts;
        Ok(state)
    }));

    pipe.register_func(Box::new(move |state: &mut State| {
        let translated = translator.translate(&state.texts.join("\n"))?;
        println!("translated: {translated:?}\n");
        state.translated = translated;

        Ok(state)
    }));

    pipe.register_func(Box::new(move |state: &mut State| {
        displayer.display(&state.translated)?;

        Ok(state)
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
