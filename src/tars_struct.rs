use bytes::Bytes;
use std::fmt::*;

pub trait TarsStruct: Debug {
    fn from_bytes(&mut self, buf: Bytes);
    fn to_bytes(&self) -> Bytes;
}