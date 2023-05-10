use honggfuzz::fuzz;
use ustr::{Ustr};

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(input_str) = std::str::from_utf8(data) {
                // Fuzz Ustr creation
                let fuzzed_ustr = Ustr::from(input_str);
        
                // Fuzz comparison and copy
                let copied_ustr = fuzzed_ustr;
                assert_eq!(fuzzed_ustr, copied_ustr);
            }
        });
    }
}