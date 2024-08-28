use bytes::{Buf, BytesMut};

pub fn buffer_to_array(buf: &mut BytesMut) -> Vec<String> {
    let mut vec = vec![];
    let len = buf.len();
    let mut word = "".to_string();

    for i in 0..len {
        match buf.get_u8() {
            b' ' => {
                vec.push(word);
                word = "".to_string();
            }
            other => {
                word.push(other as char);
                let new = word.clone();
                if i == len - 1 {
                    vec.push(new);
                }
            }
        }
    }
    return vec;
}
