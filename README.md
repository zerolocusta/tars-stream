# tars-stream
for tencent/Tars TARS Protocol encoding/decoding

# tars type 与 rust type 映射关系
|Tars Type|Rust Type|
|---------|---------|
|bool|bool|
|char|i8|
|short|i16|
|int|i32|
|long|i64|
|float|f32|
|double|f64|
|string|String|
|unsigned byte|u8(兼容 tars::Short)|
|unsigned short|u16(兼容 tars::Int32)|
|unsigned int|u32(兼容 tars::Int64)|
|vector\<char>|bytes::Bytes|
|vector<T>|Vec<T>|
|map<K, V>|BTreeMap<K, V>|
