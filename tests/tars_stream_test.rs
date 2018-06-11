extern crate bytes;
extern crate tars_stream;

use bytes::{Bytes, BytesMut};
use std::collections::BTreeMap;
use tars_stream::prelude::*;

mod common;

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
    fn decode_from_bytes(b: &Bytes) -> Result<Self, DecodeErr> {
        let mut de = TarsDecoder::new(&b);
        println!("get a");
        let a = de.get_require(0)?;
        println!("get b");
        let b = de.get_require(1)?;
        println!("get v1");
        let v1 = de.get_require(2)?;
        println!("get c");
        let c = de.get_optional(3)?;
        Ok(TestStruct{a, b, v1, c})
    }
}

impl EncodeTo for TestStruct {
    fn encode_into_bytes(&self, _tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        self.a.encode_into_bytes(0, buf)?;
        self.b.encode_into_bytes(1, buf)?;
        self.v1.encode_into_bytes(2, buf)?;
        self.c.encode_into_bytes(3, buf)?;
        Ok(())
    }
}

#[test]
fn test_encode_decode_struct() {
    let mut buf = BytesMut::new();
    let mut ts = TestStruct::new();
    ts.a = -127;
    ts.b = 12345;
    ts.v1.push(0);
    ts.v1.push(1);
    ts.v1.push(255);
    ts.c = Some("foo bar!".to_string());

    ts.encode_into_bytes(0, &mut buf).unwrap();

    let de_ts = TestStruct::decode_from_bytes(&buf.freeze()).unwrap();
    
    assert_eq!(de_ts.a, ts.a);
    assert_eq!(de_ts.b, ts.b);
    assert_eq!(de_ts.v1, ts.v1);
    assert_eq!(de_ts.c, ts.c);

}




#[derive(Clone, Debug)]
struct TestStruct2 {
    f: f32,                      // 0
    s: TestStruct,               // 1
    m: BTreeMap<String, String>, // 2
    s2: TestStruct,              // 3
    y: Option<u8>,               // 4 option
}

impl TestStruct2 {
    pub fn new() -> Self {
        TestStruct2 {
            f: 0.0,
            s: TestStruct::new(),
            m: BTreeMap::new(),
            s2: TestStruct::new(),
            y: None,
        }
    }
}

impl DecodeFrom for TestStruct2 {
    fn decode_from_bytes(b: &Bytes) -> Result<Self, DecodeErr> {
        let mut de = TarsDecoder::new(&b);
        let s = TestStruct::decode_from_bytes(&de.get_require(1)?)?;
        let s2 = TestStruct::decode_from_bytes(&de.get_require(3)?)?;
        let m = de.get_require(2)?;
        let f = de.get_require(0)?;
        let y = de.get_optional(4)?;
        Ok(TestStruct2 { f, s, m, s2, y })
    }
}

impl EncodeTo for TestStruct2 {
    fn encode_into_bytes(&self, _tag: u8, buf: &mut BytesMut) -> Result<(), EncodeErr> {
        self.f.encode_into_bytes(0, buf)?;
        write_struct(1, buf, &self.s)?;
        self.m.encode_into_bytes(2, buf)?;
        write_struct(3, buf, &self.s2)?;
        self.y.encode_into_bytes(4, buf)?;
        Ok(())
    }
}