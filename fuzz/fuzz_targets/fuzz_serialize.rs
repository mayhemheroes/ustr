use honggfuzz::fuzz;
use ustr::{Ustr};

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(input_str) = std::str::from_utf8(data) {
                // Fuzz Ustr creation
                let fuzzed_ustr = Ustr::from(input_str);
        
                // Fuzz serialization and deserialization
                let json = serde_json::to_string(&fuzzed_ustr).unwrap();
                let deserialized_ustr: Ustr = serde_json::from_str(&json).unwrap();
                assert_eq!(fuzzed_ustr, deserialized_ustr);
            }
        });
    }
}