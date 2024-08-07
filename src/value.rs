use std::collections::HashSet;

pub enum MinxValue {
    Bool(bool),
    UInt8(u8),
    Int8(i8),
    UInt16(u16),
    Int16(i16),
    UInt32(u32),
    Int32(i32),
    UInt64(u64),
    Int64(i64),
    String(String),
    Array(Vec<MinxValue>),
    Set(HashSet<MinxValue>),
}

impl MinxValue {
    pub fn serilize(&self) -> Vec<u8> {
        // TODO protobuf
    }
}
