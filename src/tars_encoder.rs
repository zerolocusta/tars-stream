// use bytes::{BufMut, BytesMut};
// use tars_type::TarsTypeMark;
// use tars_type::TarsTypeMark::*;

// #[derive(Debug)]
// pub struct TarsStructEncoder {
//     buf: BytesMut,
// }

// impl<'a> TarsStructEncoder {
//     pub fn new() -> TarsStructEncoder {
//         TarsStructEncoder {
//             buf: BytesMut::new(),
//         }
//     }

//     fn put_head(&mut self, tars_type: TarsTypeMark, tag: u8) {
//         if tag < 15 {
//             let head = (tag << 4) | tars_type.value();
//             self.buf.put_u8(head);
//         } else {
//             let head: u16 = (((0xF0u8) | tars_type.value()) as u16) << 8 | tag as u16;
//             self.buf.put_u16_be(head)
//         }
//     }

//     pub fn put_i8(&self, tag: u8, value: i8) {}

//     pub fn to_bytes_ref(&self) -> &BytesMut {
//         &self.buf
//     }
// }


// #[cfg(test)]
// mod tests {
//     use super::TarsStructEncoder;

//     #[test]
//     fn test_put_head() {
//         let en = TarsStructEncoder::new();
//         // en.put_head()
//     }   
// }

