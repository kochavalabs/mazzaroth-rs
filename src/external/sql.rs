//! Access to the query host functions

#[cfg(not(feature = "host-mock"))]
use super::externs::{_kq_query_fetch, _kq_query_run};

#[cfg(feature = "host-mock")]
pub static mut QUERY_RESULT: Option<Vec<u8>> = None;

/// Execute a Query against an uploaded schema in Mazzaroth
///
/// # Arguments
///
/// * `query` - keyquery.xdr.Query to be executed against the blockchain state
///
/// # Returns
///
///  Option<Vec<u8>>
///  * `Some(Vec<u8>)` - xdr encoded result of the query execution
///  * None - the query resulted in no results
#[cfg(not(feature = "host-mock"))]
pub fn exec(query: String) -> Option<Vec<u8>> {
    let query_bytes: Vec<u8> = query.as_bytes().to_vec();
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
pub fn exec(_query: String) -> Option<Vec<u8>> {
    unsafe { QUERY_RESULT.clone() }
}
