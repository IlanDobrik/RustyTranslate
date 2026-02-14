pub mod basic;

use anyhow::Result;

pub trait Capturer {
    fn capture(&mut self) -> Result<Vec<u8>>;
}
