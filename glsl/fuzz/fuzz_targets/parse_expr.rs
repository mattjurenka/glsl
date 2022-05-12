#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate glsl;

use glsl::syntax::Expr;
use glsl::parser::Parse as _;

fuzz_target!(|data: &[u8]| {
    match std::str::from_utf8(data) {
        Ok(s) => {
            let _ = Expr::parse(s);
        },
        _ => {},
    }
});
