use std::collections::HashMap;

use bytes::Bytes;

pub struct Db {
    entries: HashMap<String, Bytes>,
}

impl Db {
    pub fn new() -> Db {
        Db {
            entries: HashMap::new(),
        }
    }
    pub fn write(&mut self, arr: &[String]) -> Result<&str, &'static str> {
        let key = &arr[1];
        let value = &arr[2];

        let val = value.clone();
        let res: &Option<Bytes> = &self.entries.insert(String::from(key), Bytes::from(val));

        match res {
            Some(_res) => Ok("r Ok"),
            None => Ok("Ok"),
        }
    }

    pub fn read(&mut self, arr: &[String]) -> Result<&Bytes, &'static str> {
        let key = &arr[1];
        let query_res = self.entries.get(key);

        if let Some(value) = query_res {
            return Ok(value);
        } else {
            return Err("key not found");
        }
    }
}
