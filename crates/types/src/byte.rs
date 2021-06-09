pub type Byte = u8;

pub struct Bytes<'a>(pub &'a [Byte]);

impl<'a> Bytes<'a> {
    pub fn content(&self) -> &'a [u8] {
        self.0
    }
}
