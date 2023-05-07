#![no_main]

use num_format::{Locale, ToFormattedString};

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

    let _ = input.number.to_formatted_string(&locale);
});