//! Access to the query host functions

#[cfg(not(feature = "host-mock"))]
use super::externs::{_kq_insert, _kq_query_fetch, _kq_query_run};
#[cfg(not(feature = "host-mock"))]
use xdr_rs_serialize::ser::XDROut;

use keyquery::xdr::{Insert, Query};

#[cfg(feature = "host-mock")]
pub static mut QUERY_RESULT: Option<Vec<u8>> = None;

#[cfg(not(feature = "host-mock"))]
pub fn insert(insert: Insert) {
    let mut insert_bytes: Vec<u8> = Vec::new();
    insert.write_xdr(&mut insert_bytes).unwrap();
    unsafe { _kq_insert(insert_bytes.as_ptr(), insert_bytes.len()) };
}

#[cfg(feature = "host-mock")]
pub fn insert(_insert: Insert) {}

#[cfg(not(feature = "host-mock"))]
pub fn query(query: Query) -> Option<Vec<u8>> {
    let mut query_bytes: Vec<u8> = Vec::new();
    query.write_xdr(&mut query_bytes).unwrap();
    let mut hash = Vec::with_capacity(16 as usize); // 32 byte (256) hash
    unsafe { hash.set_len(16 as usize) };
    let len = unsafe { _kq_query_run(query_bytes.as_ptr(), query_bytes.len(), hash.as_ptr()) };
    if len == 0 {
        return None;
    }
    let mut result = Vec::with_capacity(len as usize);
    unsafe {
        result.set_len(len as usize);
        _kq_query_fetch(result.as_ptr(), hash.as_ptr());
    };
    return Some(result);
}

#[cfg(feature = "host-mock")]
pub fn query(_query: Query) -> Option<Vec<u8>> {
    unsafe { QUERY_RESULT.clone() }
}
