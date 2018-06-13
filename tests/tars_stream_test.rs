#[cfg(test)]
extern crate bytes;
extern crate rand;
extern crate tars_stream;
extern crate uuid;

use bytes::Bytes;
use rand::random;
use std::collections::BTreeMap;
use tars_stream::prelude::*;
use uuid::Uuid;

// mod common;

#[derive(Clone, Debug, PartialEq)]
struct TestStruct {
    a: i8,             // tag 0
    b: u16,            // tag 1
    v1: Vec<u8>,       // tag 2
    c: Option<String>, // tag 3 option
    v2: Vec<i8>,
    v3: Vec<bool>,
}

impl TestStruct {
    pub fn new() -> Self {
        TestStruct {
            a: 0,
            b: 0,
            v1: vec![],
            c: None,
            v2: vec![],
            v3: vec![],
        }
    }

    pub fn random_for_test() -> Self {
        let vec_len: u8 = random();
        let mut v1 = vec![];
        for _ in 0..vec_len {
            v1.push(random());
        }

        let vec_len: u8 = random();
        let mut v2 = vec![];
        for _ in 0..vec_len {
            v2.push(random());
        }

        let vec_len: u8 = random();
        let mut v3 = vec![];
        for _ in 0..vec_len {
            v3.push(random());
        }

        TestStruct {
            a: random(),
            b: random(),
            v1: v1,
            c: Some(Uuid::new_v4().to_string()),
            v2: v2,
            v3: v3,
        }
    }
}

impl DecodeFrom for TestStruct {
    fn decode_from(b: &Bytes) -> Result<Self, DecodeErr> {
        let mut de = TarsDecoder::new(&b);
        let a = de.get(0)?;
        let b = de.get(1)?;
        let v1 = de.get(2)?;
        let c = de.get(3)?;
        let v2 = de.get(4)?;
        let v3 = de.get(5)?;
        Ok(TestStruct {
            a,
            b,
            v1,
            c,
            v2,
            v3,
        })
    }
}

impl EncodeInto for TestStruct {
    fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        encoder.put(0, &self.a)?;
        encoder.put(1, &self.b)?;
        encoder.put(2, &self.v1)?;
        encoder.put(3, &self.c)?;
        encoder.put(4, &self.v2)?;
        encoder.put(5, &self.v3)?;
        Ok(())
    }
}

unsafe impl Sync for TestStruct {}
unsafe impl Send for TestStruct {}

#[test]
fn test_encode_decode_struct() {
    let mut encoder = TarsEncoder::new();
    let ts = TestStruct::new();

    ts.encode_into(&mut encoder).unwrap();

    let de_ts = TestStruct::decode_from(&encoder.to_bytes()).unwrap();

    assert_eq!(de_ts, ts);

    let mut encoder = TarsEncoder::new();
    let ts = TestStruct::random_for_test();

    ts.encode_into(&mut encoder).unwrap();

    let de_ts = TestStruct::decode_from(&encoder.to_bytes()).unwrap();

    assert_eq!(de_ts, ts);
}

#[derive(Clone, Debug, PartialEq)]
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

            u1: 0, // 6
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
        let f1 = de.get(0)?;
        let f2 = de.get(1)?;

        let i1 = de.get(2)?;
        let i2 = de.get(3)?;
        let i3 = de.get(4)?;
        let i4 = de.get(5)?;

        let u1 = de.get(6)?;
        let u2 = de.get(7)?;
        let u3 = de.get(8)?;
        let u4 = de.get(9)?;

        let b = de.get(10)?;

        let s = de.get(11)?;
        let v = de.get(12)?;
        let m = de.get(13)?;
        let y = de.get(14)?;
        let z = de.get(15)?;
        Ok(TestStruct2 {
            f1,
            f2,
            i1,
            i2,
            i3,
            i4,
            u1,
            u2,
            u3,
            u4,
            b,
            s,
            v,
            m,
            y,
            z,
        })
    }
}

impl EncodeInto for TestStruct2 {
    fn encode_into(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        encoder.put(0, &self.f1)?;
        encoder.put(1, &self.f2)?;

        encoder.put(2, &self.i1)?;
        encoder.put(3, &self.i2)?;
        encoder.put(4, &self.i3)?;
        encoder.put(5, &self.i4)?;

        encoder.put(6, &self.u1)?;
        encoder.put(7, &self.u2)?;
        encoder.put(8, &self.u3)?;
        encoder.put(9, &self.u4)?;

        encoder.put(10, &self.b)?;
        encoder.put(11, &self.s)?;
        encoder.put(12, &self.v)?;
        encoder.put(13, &self.m)?;
        encoder.put(14, &self.y)?;
        encoder.put(15, &self.z)?;

        Ok(())
    }
}

unsafe impl Sync for TestStruct2 {}
unsafe impl Send for TestStruct2 {}

#[test]
fn test_encode_decode_struct2() {
    let mut encoder = TarsEncoder::new();

    let mut ts = TestStruct2::new();

    ts.encode_into(&mut encoder).unwrap();

    let de_ts = TestStruct2::decode_from(&encoder.to_bytes()).unwrap();

    assert_eq!(de_ts, ts);

    // case 2

    ts.f1 = random();
    ts.f2 = random();

    ts.i1 = random();
    ts.i2 = random();
    ts.i3 = random();
    ts.i4 = random();

    ts.u1 = random();
    ts.u2 = random();
    ts.u3 = random();
    ts.u4 = random();

    ts.b = random();

    ts.s = TestStruct::random_for_test();

    let v_len: u8 = random();
    for _ in 0..v_len {
        ts.v.push(TestStruct::random_for_test());
    }

    let m_len: u8 = random();
    for _ in 0..m_len {
        ts.m
            .insert(Uuid::new_v4().to_string(), Uuid::new_v4().to_string());
    }

    ts.y = Some(random());
    ts.z = Some(TestStruct::random_for_test());

    let mut encoder = TarsEncoder::new();

    ts.encode_into(&mut encoder).unwrap();

    let de_ts = TestStruct2::decode_from(&encoder.to_bytes()).unwrap();

    assert_eq!(de_ts, ts);

    // case 3

    ts.y = None;

    let mut encoder = TarsEncoder::new();

    ts.encode_into(&mut encoder).unwrap();

    let de_ts = TestStruct2::decode_from(&encoder.to_bytes()).unwrap();

    assert_eq!(de_ts, ts);
}
