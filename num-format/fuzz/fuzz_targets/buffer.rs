#![no_main]

use num_format::{Buffer, Locale};

use libfuzzer_sys::fuzz_target;
use arbitrary::{Arbitrary, Unstructured};

#[derive(Arbitrary, Debug, Clone)]
struct FuzzInput {
    number: u64,
    locale_name: String,
}

fuzz_target!(|data: &[u8]| {
    let mut unstructured = Unstructured::new(data);
    
    let input = match FuzzInput::arbitrary(&mut unstructured) {
        Ok(input) => input,
        Err(_) => return,
    };

    let locale = match Locale::from_name(&input.locale_name) {
        Ok(locale) => locale,
        Err(_) => return,
    };

    // Create a stack-allocated buffer...
    let mut buf = Buffer::default();

    // Write 'number' into the buffer...
    buf.write_formatted(&input.number, &locale);

    // Get a view into the buffer as a &str...
    let _ = buf.as_str();
});