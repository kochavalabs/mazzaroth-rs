//! Access to the query host functions

#[cfg(not(feature = "host-mock"))]
use super::externs::{_kq_json_insert, _kq_query_fetch, _kq_query_run};

#[cfg(feature = "host-mock")]
pub static mut QUERY_RESULT: Option<Vec<u8>> = None;

#[cfg(feature = "host-mock")]
pub static mut INSERT_RESULT: Result<u32, u32> = Ok(0);

/// Execute a string query against the Mazzaroth leger.
///
/// # Arguments
///
/// * `query` - String query to be executed against the kvquery prefix
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

/// Executes a query that will insert a JSON object into the specified table.
///
/// # Arguments
///
/// * `table_name` - String name of the table to insert into
/// * `json` - String JSON string to insert into the table
///
/// # Returns
///
///  Result<u32, u32>
///  * Ok(x) - resulting return code upon success
///  * Err(x) - resulting return code upon error
#[cfg(not(feature = "host-mock"))]
pub fn insert(table_name: String, json: String) -> Result<u32, u32> {
    let json_bytes: Vec<u8> = json.as_bytes().to_vec();
    let table_bytes: Vec<u8> = table_name.as_bytes().to_vec();
    match unsafe {
        _kq_json_insert(
            table_bytes.as_ptr(),
            table_bytes.len(),
            json_bytes.as_ptr(),
            json_bytes.len(),
        )
    } {
        0 => Ok(0),
        x => Err(x),
    }
}

#[cfg(feature = "host-mock")]
pub fn insert(_table_name: String, _json: String) -> Result<u32, u32> {
    unsafe { INSERT_RESULT.clone() }
}
