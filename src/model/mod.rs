mod error;

pub use self::error::{Error, Result};

#[derive(Clone)]
pub struct ModelManager {}

impl ModelManager {
    pub fn new() -> Result<Self> {
        Ok(ModelManager {})
    }
}
