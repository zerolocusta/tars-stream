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

#[derive(Clone, Debug, PartialEq)]
struct TestStruct {
    a: i8,             // tag 0
    b: u16,            // tag 1
    v1: Vec<u8>,       // tag 2
    c: String, // tag 3 option
    v2: Vec<i8>,
    v3: Vec<bool>,
}

impl TestStruct {
    pub fn new() -> Self {
        TestStruct {
            a: 0,
            b: 0,
            v1: vec![],
            c: String::from("hello world"),
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
            c: Uuid::new_v4().to_string(),
            v2: v2,
            v3: v3,
        }
    }
}

impl DecodeFromTars for TestStruct {
    fn decode_from_tars(decoder: &TarsDecoder, _tag: u8) -> Result<Self, DecodeErr> {
        let a = decoder.read_int8(0, true, 0)?;
        let b = decoder.read_uint16(1, true, 0)?;
        let v1 = decoder.read_list(2, true, vec![])?;
        let c = decoder.read_string(3, false, "hello world".to_string())?;
        let v2 = decoder.read_list(4, true, vec![])?;
        let v3 = decoder.read_list(5, true, vec![])?;
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

impl EncodeIntoTars for TestStruct {
    fn encode_into_tars(&self, encoder: &mut TarsEncoder, _tag: u8) -> Result<(), EncodeErr> {
        encoder.write_int8(0, self.a)?;
        encoder.write_uint16(1, self.b)?;
        encoder.write_list(2, &self.v1)?;
        encoder.write_string(3, &self.c)?;
        encoder.write_list(4, &self.v2)?;
        encoder.write_list(5, &self.v3)?;
        Ok(())
    }
}

impl ClassName for TestStruct {
    fn _class_name() -> String {
        String::from("TarsStreamTest.TestStruct")
    }
}

#[test]
fn test_encode_decode_struct() {
    let mut encoder = TarsEncoder::new();
    let ts = TestStruct::new();

    ts.encode_into_tars(&mut encoder, 0).unwrap();

    let decoder = TarsDecoder::from(&encoder.to_bytes());

    let de_ts = TestStruct::decode_from_tars(&decoder, 0).unwrap();

    assert_eq!(de_ts, ts);

    let mut encoder = TarsEncoder::new();
    let ts = TestStruct::random_for_test();

    ts.encode_into_tars(&mut encoder, 0).unwrap();

    let decoder = TarsDecoder::from(&encoder.to_bytes());

    let de_ts = TestStruct::decode_from_tars(&decoder, 0).unwrap();

    assert_eq!(de_ts, ts);
}

// #[derive(Clone, Debug, PartialEq)]
// enum TestEnum {
//     A = -32,
//     B = 1337,
// }

// impl DecodeFromTars for TestEnum {
//     fn decode_from_tars(b: &Bytes) -> Result<Self, DecodeErr> {
//         match i32::decode_from_tars(b)? {
//             -32 => Ok(TestEnum::A),
//             1337 => Ok(TestEnum::B),
//             _ => Err(DecodeErr::InvalidEnumValue),
//         }
//     }
// }

// impl EncodeIntoTars for TestEnum {
//     fn encode_into_tars(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
//         (self.clone() as i32).encode_into_tars(encoder)
//     }
// }

// impl ClassName for TestEnum {
//     fn _class_name() -> String {
//         String::from("TestEnum")
//     }
// }

// #[derive(Clone, Debug, PartialEq)]
// struct TestStruct2 {
//     f1: f32, // 0
//     f2: f64, // 1

//     i1: i8,  // 2
//     i2: i16, // 3
//     i3: i32, // 4
//     i4: i64, // 5

//     u1: u8,  // 6
//     u2: u16, // 7
//     u3: u32, // 8

//     b: bool, // 10

//     s: TestStruct,               // 11
//     v: Vec<TestStruct>,          // 12
//     m: BTreeMap<String, String>, // 13
//     y: Option<u8>,               // 14
//     z: Option<TestStruct>,       // 15
//     x: Bytes,
//     e: Vec<TestEnum>,
// }

// impl TestStruct2 {
//     pub fn new() -> Self {
//         TestStruct2 {
//             f1: 0.0,
//             f2: 0.0,

//             i1: 0, // 2
//             i2: 0, // 3
//             i3: 0, // 4
//             i4: 0, // 5

//             u1: 0, // 6
//             u2: 0, // 7
//             u3: 0, // 8

//             b: false,

//             s: TestStruct::new(),
//             v: vec![],
//             m: BTreeMap::new(),
//             y: None,
//             z: None,
//             x: Bytes::new(),
//             e: vec![],
//         }
//     }
// }

// impl DecodeFromTars for TestStruct2 {
//     fn decode_from_tars(b: &Bytes) -> Result<Self, DecodeErr> {
//         let mut de = TarsDecoder::from(b);
//         let f1 = de.get(0)?;
//         let f2 = de.get(1)?;

//         let i1 = de.get(2)?;
//         let i2 = de.get(3)?;
//         let i3 = de.get(4)?;
//         let i4 = de.get(5)?;

//         let u1 = de.get(6)?;
//         let u2 = de.get(7)?;
//         let u3 = de.get(8)?;

//         let b = de.get(10)?;

//         let s = de.get(11)?;
//         let v = de.get(12)?;
//         let m = de.get(13)?;
//         let y = de.get(14)?;
//         let z = de.get(15)?;
//         let x = de.get(16)?;
//         let e = de.get(17)?;

//         Ok(TestStruct2 {
//             f1,
//             f2,
//             i1,
//             i2,
//             i3,
//             i4,
//             u1,
//             u2,
//             u3,
//             b,
//             s,
//             v,
//             m,
//             y,
//             z,
//             x,
//             e,
//         })
//     }
// }

// impl EncodeIntoTars for TestStruct2 {
//     fn encode_into_tars(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
//         encoder.put(0, &self.f1)?;
//         encoder.put(1, &self.f2)?;

//         encoder.put(2, &self.i1)?;
//         encoder.put(3, &self.i2)?;
//         encoder.put(4, &self.i3)?;
//         encoder.put(5, &self.i4)?;

//         encoder.put(6, &self.u1)?;
//         encoder.put(7, &self.u2)?;
//         encoder.put(8, &self.u3)?;

//         encoder.put(10, &self.b)?;
//         encoder.put(11, &self.s)?;
//         encoder.put(12, &self.v)?;
//         encoder.put(13, &self.m)?;
//         encoder.put(14, &self.y)?;
//         encoder.put(15, &self.z)?;
//         encoder.put(16, &self.x)?;
//         encoder.put(17, &self.e)?;

//         Ok(())
//     }
// }

// impl ClassName for TestStruct2 {
//     fn _class_name() -> String {
//         String::from("TestStruct2")
//     }
// }

// #[test]
// fn test_encode_decode_struct2() {
//     let mut encoder = TarsEncoder::new();

//     let mut ts = TestStruct2::new();

//     ts.encode_into_tars(&mut encoder).unwrap();

//     let de_ts = TestStruct2::decode_from_tars(&encoder.to_bytes()).unwrap();

//     assert_eq!(de_ts, ts);

//     // case 2

//     ts.f1 = random();
//     ts.f2 = random();

//     ts.i1 = random();
//     ts.i2 = random();
//     ts.i3 = random();
//     ts.i4 = random();

//     ts.u1 = random();
//     ts.u2 = random();
//     ts.u3 = random();

//     ts.b = random();

//     ts.s = TestStruct::random_for_test();

//     let v_len: u8 = random();
//     for _ in 0..v_len {
//         ts.v.push(TestStruct::random_for_test());
//     }

//     let m_len: u8 = random();
//     for _ in 0..m_len {
//         ts.m
//             .insert(Uuid::new_v4().to_string(), Uuid::new_v4().to_string());
//     }

//     ts.y = Some(random());
//     ts.z = Some(TestStruct::random_for_test());
//     ts.x = Bytes::from(ts.s.v1.as_slice());

//     let e_len: u8 = random();
//     for _ in 0..e_len {
//         let b: bool = random();
//         if b {
//             ts.e.push(TestEnum::A);
//         } else {
//             ts.e.push(TestEnum::B);
//         }
//     }
//     let mut encoder = TarsEncoder::new();

//     ts.encode_into_tars(&mut encoder).unwrap();

//     let de_ts = TestStruct2::decode_from_tars(&encoder.to_bytes()).unwrap();

//     assert_eq!(de_ts, ts);

//     // case 3

//     ts.y = None;

//     let mut encoder = TarsEncoder::new();

//     ts.encode_into_tars(&mut encoder).unwrap();

//     let de_ts = TestStruct2::decode_from_tars(&encoder.to_bytes()).unwrap();

//     assert_eq!(de_ts, ts);
// }
