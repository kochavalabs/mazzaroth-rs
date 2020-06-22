# mazzaroth-rs

[![CircleCI](https://circleci.com/gh/kochavalabs/mazzaroth-rs.svg?style=svg)](https://circleci.com/gh/kochavalabs/mazzaroth-rs)

The Mazzaroth Rust Library is a rust library that includes host bindings
and everything needed to compile rust contracts to Web Assembly, compatible
with the Mazzaroth VM.  Here you will find the necessary abi encoders and
decoders used to pass and return arguments to contract functions as well as
the external host functions available to use.

## Usage

The first step to using this library is to include the necessary dependencies.  
The following 3 dependencies should be included in your Cargo.toml:

mazzaroth-rs  
mazzaroth-rs-derive  
mazzaroth-xdr

Every contract will have a similar base layout for the main function and the contract trait definition.
`main()` is used as the entry point and has several important features.  It will instantiate the contract,
call a host function to retrieve function input, execute the function, and return a response.

Here is a basic Hello World contract example:

```rust
// must include the ContractInterface and mazzaroth_abi for compiling the macro
extern crate mazzaroth_rs;
extern crate mazzaroth_derive;
use mazzaroth_rs::ContractInterface;
use mazzaroth_rs_derive::mazzaroth_abi;

// using specific external host modules
use mazzaroth_rs::external::{transaction, log};

#[no_mangle]
pub fn main() {
    // panic hook is set to call the host error log function when a panic occurs
    std::panic::set_hook(Box::new(mazzaroth_rs::external::errors::hook));

    // Creates a new instance of the ABI generated around the Contract
    let mut contract = HelloWorld::new(Hello {});

    // Use a host function to get arguments
    let args = transaction::arguments();

    // Execute calls one of the functions defined in the contract
    // Input for the function to call and it's params comes from the Runtime
    let response = contract.execute(&args);

    // Provide return value through host call
    transaction::ret(response);
}

// mazzaroth_abi used to generate the contract from the trait during compilation
#[mazzaroth_abi(HelloWorld)]
pub trait HelloWorldContract {
    // hello() defined as a readonly function
    #[readonly]
    fn hello(&mut self) -> u32;
}

// Struct used to implement the contract trait
pub struct Hello {}

// Actual contract implementation
impl HelloWorldContract for Hello {
    fn hello(&mut self) -> u32 {
        log("Hello World!".to_string());
        14
    }
}
```

## Generating Documentation

From the root directory run the command:

```console
cargo doc
```

Optionally provide the --open flag to open the docs in a browser after building them.

This will put all generated docs in `/target/doc`

## License

[MIT](https://choosealicense.com/licenses/mit/)
