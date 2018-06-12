extern crate bytes;
extern crate tars_stream;

use bytes::Bytes;
use std::collections::BTreeMap;
use tars_stream::prelude::*;

// mod common;

#[derive(Clone, Debug)]
struct TestStruct {
    a: i8,             // tag 0
    b: u16,            // tag 1
    v1: Vec<u8>,       // tag 2
    c: Option<String>, // tag 3 option
}

impl TestStruct {
    pub fn new() -> Self {
        TestStruct {
            a: 0,
            b: 0,
            v1: vec![],
            c: None,
        }
    }
}

impl DecodeFrom for TestStruct {
    fn decode_from(b: &Bytes) -> Result<Self, DecodeErr> {
        let mut de = TarsDecoder::new(&b);
        let a = de.get_require(0)?;
        let b = de.get_require(1)?;
        let v1 = de.get_require(2)?;
        let c = de.get(3)?;
        Ok(TestStruct { a, b, v1, c })
    }
}

impl EncodeTo for TestStruct {
    fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        encoder.put(0, &self.a)?;
        encoder.put(1, &self.b)?;
        encoder.put(2, &self.v1)?;
        encoder.put(3, &self.c)?;
        Ok(())
    }
}

#[test]
fn test_encode_decode_struct() {
    let mut encoder = TarsEncoder::new();
    let mut ts = TestStruct::new();
    ts.a = -127;
    ts.b = 12345;
    ts.v1.push(0);
    ts.v1.push(1);
    ts.v1.push(255);
    ts.c = Some("foo bar!".to_string());

    ts.encode_into(&mut encoder).unwrap();

    let de_ts = TestStruct::decode_from(&encoder.to_bytes()).unwrap();

    assert_eq!(de_ts.a, ts.a);
    assert_eq!(de_ts.b, ts.b);
    assert_eq!(de_ts.v1, ts.v1);
    assert_eq!(de_ts.c, ts.c);
}

#[derive(Clone, Debug)]
struct TestStruct2 {
    f1: f32, // 0
    f2: f64, // 1

    i1: i8,  // 2
    i2: i16, // 3
    i3: i32, // 4
    i4: i64, // 5

    u1: u8,  // 6
    u2: u16, // 7
    u3: u32, // 8
    u4: u64, // 9

    b: bool, // 10

    s: TestStruct,               // 11
    v: Vec<TestStruct>,          // 12
    m: BTreeMap<String, String>, // 13
    y: Option<u8>,               // 14
    z: Option<TestStruct>,       // 15
}

impl TestStruct2 {
    pub fn new() -> Self {
        TestStruct2 {
            f1: 0.0,
            f2: 0.0,

            i1: 0, // 2
            i2: 0, // 3
            i3: 0, // 4
            i4: 0, // 5

            u1: 0,  // 6
            u2: 0, // 7
            u3: 0, // 8
            u4: 0, // 9

            b: false,

            s: TestStruct::new(),
            v: vec![],
            m: BTreeMap::new(),
            y: None,
            z: None,
        }
    }
}

impl DecodeFrom for TestStruct2 {
    fn decode_from(b: &Bytes) -> Result<Self, DecodeErr> {
        let mut de = TarsDecoder::new(&b);
        let f1 = de.get_require(0)?;
        let f2 = de.get_require(1)?;

        let i1 = de.get_require(0)?;
        let i2 = de.get_require(0)?;
        let i3 = de.get_require(0)?;
        let i4 = de.get_require(0)?;

        let u1 = de.get_require(0)?;
        let u2 = de.get_require(0)?;
        let u3 = de.get_require(0)?;
        let u4 = de.get_require(0)?;

        let b = de.get_require(0)?;

        let s = de.get_require(0)?;
        let v = de.get_require(0)?;
        let m = de.get_require(0)?;
        let y = de.get_optional(0)?;
        let z = de.get_optional(0)?;
        Ok(TestStruct2 { f1, f2, i1, i2, i3, i4, u1, u2, u3, u4, b, s, v, m, y, z})
    }
}

// // impl EncodeTo for TestStruct2 {
// //     fn encode_into_bytes(&self, _tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
// //         self.f1.encode_into_bytes(0, buf)?;
// //         write_struct(1, buf, &self.s)?;
// //         self.m.encode_into_bytes(2, buf)?;
// //         write_struct(3, buf, &self.s2)?;
// //         self.y.encode_into_bytes(4, buf)?;
// //         Ok(())
// //     }
// // }
