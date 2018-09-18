use Result;

pub struct Game {}

impl Game {
    pub fn score(_rolls: &[u8]) -> Option<u16> {
        None
    }

    pub fn validate(rolls: &[u8]) -> Result<&[u8]> {
        Ok(rolls)
    }
}
