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

#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct TestStruct {
    a: i8,       // tag 0
    b: u16,      // tag 1
    v1: Vec<u8>, // tag 2
    c: String,   // tag 3 option
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

impl StructEncodeIntoTars for TestStruct {
    fn struct_encode_into_tars(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        encoder.write_int8(0, self.a)?;
        encoder.write_uint16(1, self.b)?;
        encoder.write_list(2, &self.v1)?;
        encoder.write_string(3, &self.c)?;
        encoder.write_list(4, &self.v2)?;
        encoder.write_list(5, &self.v3)?;
        Ok(())
    }
}

impl StrcutDecodeFromTars for TestStruct {
    fn struct_decode_from_tars(decoder: &mut TarsDecoder) -> Result<Self, DecodeErr> {
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

impl DecodeFromTars for TestStruct {
    fn decode_from_tars(decoder: &mut TarsDecoder, tag: u8) -> Result<Self, DecodeErr> {
        decoder.read_struct(tag, true, TestStruct::new())
    }
}

impl EncodeIntoTars for TestStruct {
    fn encode_into_tars(&self, encoder: &mut TarsEncoder, tag: u8) -> Result<(), EncodeErr> {
        encoder.write_struct(tag, self)
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

    let mut decoder = TarsDecoder::from(&encoder.to_bytes());

    let de_ts = TestStruct::decode_from_tars(&mut decoder, 0).unwrap();

    assert_eq!(de_ts, ts);

    let mut encoder = TarsEncoder::new();
    let ts = TestStruct::random_for_test();

    ts.encode_into_tars(&mut encoder, 0).unwrap();

    let mut decoder = TarsDecoder::from(&encoder.to_bytes());

    let de_ts = TestStruct::decode_from_tars(&mut decoder, 0).unwrap();

    assert_eq!(de_ts, ts);
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum TestEnum {
    A = -32,
    B = 1337,
}

impl DecodeFromTars for TestEnum {
    fn decode_from_tars(decoder: &mut TarsDecoder, tag: u8) -> Result<Self, DecodeErr> {
        match decoder.read_int32(tag, true, -32)? {
            -32 => Ok(TestEnum::A),
            1337 => Ok(TestEnum::B),
            _ => Err(DecodeErr::InvalidEnumValue),
        }
    }
}

impl EncodeIntoTars for TestEnum {
    fn encode_into_tars(&self, encoder: &mut TarsEncoder, tag: u8) -> Result<(), EncodeErr> {
        encoder.write_int32(tag, (self.clone()) as i32)
    }
}

impl ClassName for TestEnum {
    fn _class_name() -> String {
        String::from("TarsStreamTest.TestEnum")
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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

    b: bool, // 10

    s: TestStruct,               // 11
    v: Vec<TestStruct>,          // 12
    m: BTreeMap<String, String>, // 13
    y: u8,                       // 14
    z: TestStruct,               // 15
    x: Bytes,
    e: Vec<TestEnum>,
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

            b: false,

            s: TestStruct::new(),
            v: vec![],
            m: BTreeMap::new(),
            y: 0,
            z: TestStruct::new(),
            x: Bytes::new(),
            e: vec![],
        }
    }

    pub fn random_for_test() -> Self {
        let mut ts = TestStruct2::new();

        ts.f1 = random();
        ts.f2 = random();

        ts.i1 = random();
        ts.i2 = random();
        ts.i3 = random();
        ts.i4 = random();

        ts.u1 = random();
        ts.u2 = random();
        ts.u3 = random();

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

        ts.y = random();
        ts.z = TestStruct::random_for_test();
        ts.x = Bytes::from(ts.s.v1.as_slice());

        let e_len: u8 = random();
        for _ in 0..e_len {
            let b: bool = random();
            if b {
                ts.e.push(TestEnum::A);
            } else {
                ts.e.push(TestEnum::B);
            }
        }

        ts
    }
}

impl DecodeFromTars for TestStruct2 {
    fn decode_from_tars(decoder: &mut TarsDecoder, tag: u8) -> Result<Self, DecodeErr> {
        decoder.read_struct(tag, true, Self::new())
    }
}

impl StrcutDecodeFromTars for TestStruct2 {
    fn struct_decode_from_tars(decoder: &mut TarsDecoder) -> Result<Self, DecodeErr> {
        let f1 = decoder.read_float(0, true, 0.0)?;
        let f2 = decoder.read_double(1, true, 0.0)?;

        let i1 = decoder.read_int8(2, true, 0)?;
        let i2 = decoder.read_int16(3, true, 0)?;
        let i3 = decoder.read_int32(4, true, 0)?;
        let i4 = decoder.read_int64(5, true, 0)?;

        let u1 = decoder.read_uint8(6, true, 0)?;
        let u2 = decoder.read_uint16(7, true, 0)?;
        let u3 = decoder.read_uint32(8, true, 0)?;

        let b = decoder.read_boolean(9, true, false)?;

        let s = decoder.read_struct(10, true, TestStruct::new())?;
        let v = decoder.read_list(11, true, vec![])?;
        let m = decoder.read_map(12, true, BTreeMap::new())?;
        let y = decoder.read_uint8(13, false, 0)?;
        let z = decoder.read_struct(14, false, TestStruct::new())?;
        let x = decoder.read_bytes(15, true, Bytes::new())?;
        let e = decoder.read_list(16, true, vec![])?;

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
            b,
            s,
            v,
            m,
            y,
            z,
            x,
            e,
        })
    }
}

impl EncodeIntoTars for TestStruct2 {
    fn encode_into_tars(&self, encoder: &mut TarsEncoder, tag: u8) -> Result<(), EncodeErr> {
        encoder.write_struct(tag, self)
    }
}

impl StructEncodeIntoTars for TestStruct2 {
    fn struct_encode_into_tars(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        encoder.write_float(0, self.f1)?;
        encoder.write_double(1, self.f2)?;

        encoder.write_int8(2, self.i1)?;
        encoder.write_int16(3, self.i2)?;
        encoder.write_int32(4, self.i3)?;
        encoder.write_int64(5, self.i4)?;

        encoder.write_uint8(6, self.u1)?;
        encoder.write_uint16(7, self.u2)?;
        encoder.write_uint32(8, self.u3)?;

        encoder.write_boolean(9, self.b)?;
        encoder.write_struct(10, &self.s)?;
        encoder.write_list(11, &self.v)?;
        encoder.write_map(12, &self.m)?;
        encoder.write_uint8(13, self.y)?;
        encoder.write_struct(14, &self.z)?;
        encoder.write_bytes(15, &self.x)?;
        encoder.write_list(16, &self.e)?;

        Ok(())
    }
}

impl ClassName for TestStruct2 {
    fn _class_name() -> String {
        String::from("TestStruct2")
    }
}

#[test]
fn test_encode_decode_struct2() {
    let mut encoder = TarsEncoder::new();

    let mut ts = TestStruct2::new();

    ts.encode_into_tars(&mut encoder, 0).unwrap();

    let mut decoder = TarsDecoder::from(&encoder.to_bytes());

    let de_ts = TestStruct2::decode_from_tars(&mut decoder, 0).unwrap();

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

    ts.y = random();
    ts.z = TestStruct::random_for_test();
    ts.x = Bytes::from(ts.s.v1.as_slice());

    let e_len: u8 = random();
    for _ in 0..e_len {
        let b: bool = random();
        if b {
            ts.e.push(TestEnum::A);
        } else {
            ts.e.push(TestEnum::B);
        }
    }
    let mut encoder = TarsEncoder::new();

    ts.encode_into_tars(&mut encoder, 0).unwrap();

    let mut decoder = TarsDecoder::from(&encoder.to_bytes());

    let de_ts = TestStruct2::decode_from_tars(&mut decoder, 0).unwrap();

    assert_eq!(de_ts, ts);

    // case 3

    ts.y = 0;

    let mut encoder = TarsEncoder::new();

    ts.encode_into_tars(&mut encoder, 0).unwrap();

    let mut decoder = TarsDecoder::from(&encoder.to_bytes());

    let de_ts = TestStruct2::decode_from_tars(&mut decoder, 0).unwrap();

    assert_eq!(de_ts, ts);
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct TestOptionalStruct {
    a: TestStruct2,
    b: Bytes,
    c: f64,
    d: BTreeMap<String, String>,
}

impl TestOptionalStruct {
    fn new() -> Self {
        TestOptionalStruct {
            a: TestStruct2::new(),
            b: Bytes::new(),
            c: 0.0,
            d: BTreeMap::new(),
        }
    }
}

impl DecodeFromTars for TestOptionalStruct {
    fn decode_from_tars(decoder: &mut TarsDecoder, tag: u8) -> Result<Self, DecodeErr> {
        decoder.read_struct(tag, true, Self::new())
    }
}

impl StrcutDecodeFromTars for TestOptionalStruct {
    fn struct_decode_from_tars(decoder: &mut TarsDecoder) -> Result<TestOptionalStruct, DecodeErr> {
        println!("reading 0");
        let a = decoder.read_struct(0, false, TestStruct2::new())?;
        println!("reading 1");
        let b = decoder.read_bytes(1, true, Bytes::new())?;
        println!("reading 2");
        let c = decoder.read_double(2, false, 0.0)?;
        println!("reading 3");
        let d = decoder.read_map(3, true, BTreeMap::new())?;
        Ok(TestOptionalStruct { a, b, c, d })
    }
}

impl EncodeIntoTars for TestOptionalStruct {
    fn encode_into_tars(&self, encoder: &mut TarsEncoder, tag: u8) -> Result<(), EncodeErr> {
        encoder.write_struct(tag, self)
    }
}

impl StructEncodeIntoTars for TestOptionalStruct {
    fn struct_encode_into_tars(&self, encoder: &mut TarsEncoder) -> Result<(), EncodeErr> {
        // write fake binary into encoder for test skip_field

        encoder.write_int8(128, i8::min_value())?;
        encoder.write_int16(129, i16::min_value())?;
        encoder.write_int32(130, i32::min_value())?;
        encoder.write_int64(131, i64::min_value())?;

        encoder.write_uint8(132, u8::max_value())?;
        encoder.write_uint16(133, u16::max_value())?;
        encoder.write_uint32(134, u32::max_value())?;

        encoder.write_boolean(135, true)?;
        encoder.write_float(136, 7.123)?;
        encoder.write_double(137, 0.42222)?;

        encoder.write_bytes(138, &Bytes::new())?;
        encoder.write_struct(139, &TestStruct2::random_for_test())?;

        // Not write a
        encoder.write_bytes(1, &self.b)?;
        // Not write c
        encoder.write_map(3, &self.d)
    }
}
#[test]
fn test_encode_decode_optioal() {
    let s = TestOptionalStruct::new();
    let mut encoder = TarsEncoder::new();
    s.struct_encode_into_tars(&mut encoder).unwrap();
    let buf = encoder.to_bytes();
    println!("{:?}", buf);
    let mut decoder = TarsDecoder::from(&buf);
    let de_s = TestOptionalStruct::struct_decode_from_tars(&mut decoder).unwrap();

    assert_eq!(s, de_s);
}
