/// Panic hook for host runtime that has a _log_error function defined.
/// https://github.com/rustwasm/console_error_panic_hook
#[cfg(target_arch = "wasm32")]
use super::externs::_log_error;

use std::panic;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        fn hook_impl(info: &panic::PanicInfo) {
            unsafe { _log_error(info.to_string()) };
        }
    } else {
        use std::io::{self, Write};

        fn hook_impl(info: &panic::PanicInfo) {
            let _ = writeln!(io::stderr(), "{}", info);
        }
    }
}

/// A panic hook that logs panics to extern _error when building with wasm32
pub fn hook(info: &panic::PanicInfo) {
    hook_impl(info);
}
