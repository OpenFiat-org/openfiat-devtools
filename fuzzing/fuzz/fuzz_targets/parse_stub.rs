#![no_main]
use libfuzzer_sys::fuzz_target;
use openfiat_fuzzing::parse_stub;

fuzz_target!(|data: &[u8]| {
    let _ = parse_stub(data);
});
