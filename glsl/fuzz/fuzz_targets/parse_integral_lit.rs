#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate glsl;

use glsl::parsers::integral_lit;


fuzz_target!(|data: &[u8]| {
  integral_lit(data);
});
