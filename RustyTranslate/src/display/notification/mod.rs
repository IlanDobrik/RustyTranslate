use super::Displayer;
use anyhow::Result;
use notify_rust::Timeout;

pub struct Notification;

impl Displayer for Notification {
    fn display(&self, text: &str) -> Result<()> {
        notify_rust::Notification::new()
            .summary("RustyTranslate")
            .body(text)
            .timeout(Timeout::Milliseconds(6000))
            .show()?;

        Ok(())
    }
}

impl Notification {
    pub fn new() -> Self {
        Self {}
    }
}
