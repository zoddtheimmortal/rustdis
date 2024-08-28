use std::collections::HashMap;

use bytes::Bytes;

pub struct Db {
    entries: HashMap<String, Bytes>,
}
