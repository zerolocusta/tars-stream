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

# tars 协议的坑

* string 以 1 或 4 字节表示长度
* Nodejs 实现中，tag 必须按写入顺序读取，否则查找不到 tag
* optional 即使不设值（Rust使用Option表示完全没问题），其他实现中也会对 optional 给予默认值，导致 optional 只能用于兼容老版本协议，而不能用具 optional 字段鉴别
* tars::UInt8 以 tars::Int16 表示，tars::UInt16 以 tars::Int32 表示，tars::UInt32 以 tars::Int64 表示
* map 和 list 以 5 个字节的 tars::Int32 (包括头部1个字节及i32的4个字节)，表示元素个数(map 是 （key, value) 个数)...