use super::{_get,get_length,_store};

pub fn get(key: &str) -> Vec<u8> {
    let key = key.as_bytes();
    let len = unsafe { get_length(&key.to_vec()) };
    let mut val = Vec::with_capacity(len as usize);
    unsafe { val.set_len(len as usize) };
    unsafe { _get(&key.to_vec(), &val) };
    val
}

pub fn store(key: &str, val: Vec<u8>) {
    let key = key.as_bytes();
    unsafe { _store(key.as_ptr(), key.len(), val.as_ptr(), val.len()) };
}