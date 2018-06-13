use bytes::Bytes;
use std::collections::BTreeMap;

use errors::DecodeErr;
use tars_decoder::{DecodeFromTars, TarsDecoder, TarsDecoderTrait};
use tars_type::ProtocolVersion;

type SimpleTup = BTreeMap<String, Bytes>;
type ComplexTup = BTreeMap<String, BTreeMap<String, Bytes>>;

pub struct TupDecoder<M> {
    map: M,
}
// for TupSimple protocol version
impl TupDecoder<SimpleTup> {
    pub fn new() -> Self {
        TupDecoder {
            map: BTreeMap::new(),
        }
    }
}

impl<'a, K, V> From<&'a Bytes> for TupDecoder<BTreeMap<K, V>>
where
    K: DecodeFromTars + Ord,
    V: DecodeFromTars
{
    fn from(buf: &'a Bytes) -> Self {
        let mut decoder = TarsDecoder::from(buf);
        match decoder.get(0) {
            Err(_) => TupDecoder {
                map: BTreeMap::new(),
            },
            Ok(m) => TupDecoder {
                map: m,
            }
        }
    }
}

pub trait TupDecoderTrait<T> {
    fn get(&mut self, name: &String) -> Result<T, DecodeErr>;
}

impl<T> TupDecoderTrait<T> for TupDecoder<SimpleTup>
where
    T: DecodeFromTars,
{
    fn get(&mut self, name: &String) -> Result<T, DecodeErr> {
        match self.map.get(name) {
            None => Err(DecodeErr::FieldNotFoundErr),
            Some(b) => {
                T::decode_from(b)
            }
        }
    }
}
