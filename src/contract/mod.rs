/// This trait defines the execute function that can be called on a contract.
/// The implementation is generated by the derive macro but this trait must
/// be included by the contract implementation.
///
/// Example:
/// ```
/// use mazzaroth_wasm::ContractInterface;
///
/// #[no_mangle]
/// pub fn main() {
///    let mut contract = HelloWorld::new(Hello {});
///    let args = transaction::arguments();
///
///    let response = contract.execute(&args);
///
///    transaction::ret(response);
/// }
/// ```
pub trait ContractInterface {
    fn execute(&mut self, payload: &[u8]) -> Vec<u8>;
}
