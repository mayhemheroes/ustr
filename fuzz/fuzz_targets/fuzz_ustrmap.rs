use honggfuzz::fuzz;
use ustr::{Ustr, UstrMap};

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(input_str) = std::str::from_utf8(data) {
                // Fuzz Ustr creation
                let fuzzed_ustr = Ustr::from(input_str);
                
                // Fuzz UstrMap
                let mut map: UstrMap<usize> = UstrMap::default();
                map.insert(fuzzed_ustr, 17);
                assert_eq!(*map.get(&fuzzed_ustr).unwrap(), 17);
            }
        });
    }
}