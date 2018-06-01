pub trait TarsStruct {
    fn from_slice(&mut self, buf: &[u8]);
    fn to_slice(&self) -> &[u8];
}
