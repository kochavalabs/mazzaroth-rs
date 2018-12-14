/// This trait defines a function that will be called on a contract.
/// The implementation is generated by the derive Macro but this must
/// be included by the contract implementation.
pub trait ContractInterface {
    fn execute(&mut self, payload: &[u8]) -> Vec<u8>;
}