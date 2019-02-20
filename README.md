# mazzaroth-wasm-rs
mazzaroth rust library

### How to use

This library can be used by either cloning the repo and including the file path to its location in Cargo.toml dependencies or by using git dependencies:
```
mazzaroth-wasm = { git = "ssh://git@github.com/kochavalabs/mazzaroth-wasm-rs.git", branch = "develop"}
mazzaroth-wasm-derive = { git = "ssh://git@github.com/kochavalabs/mazzaroth-wasm-rs.git", branch = "develop"}
```

To use the derive proc macro to generate the mazzaroth abi include the use statement: `use mazzaroth_wasm_derive::mazzaroth_abi;` and put `#[mazzaroth_abi(<contract name>)]` above the contract trait.  For Example:
```
#[mazzaroth_abi(MRC20)]
pub trait MRC20Contract {
    // NOTE: We currently make no restriction on constructor call
    fn constructor(&mut self, owner_address: Vec<u8>, preallocated: u32);

    // NOTE: We are not pulling sender address from transaction so it must be passed in to this function
    // TODO: Boolean return ABIType
    fn transfer(&mut self, from_address: Vec<u8>, to_address: Vec<u8>, value: u32);

    fn balance_of(&mut self, address: Vec<u8>) -> u32;

    fn total_supply(&mut self) -> u32;
}
```

This proc macro will parse the trait for functions and create a new struct with an execute function that takes a Vec<u8> of the encoded arguments.  

For this example you could use the contract and pass encoded arguments with the following code:
```
    // Creates a new instance of the ABI generated around the Contract
    let mut contract = MRC20::new(Token {});

    // Get the arguments from the external host
    let args = transaction::arguments();

    // Execute calls one of the functions defined in our contract
    // Input for the function to call and it's params comes from the Runtime
    let response = contract.execute(&args);
```