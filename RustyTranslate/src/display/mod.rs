pub mod notification;

use anyhow::Result;

pub trait Displayer {
    fn display(&self, text: &str) -> Result<()>;
}
