#[allow(dead_code)]
use sp_std::vec::Vec;

use core::convert::AsRef;

fn is_unreserved_character(char: &u8) -> bool {
    match *char {
        b'-' | b'_' | b'.' | b'~' => true,
        char => (char as char).is_ascii_alphanumeric(),
    }
}

fn hex_code(char: u8) -> u8 {
    if char < 10 {
        char + b'0'
    } else {
        char - 10 + b'A'
    }
}

pub fn percent_encode<T: AsRef<[u8]>>(string: T) -> Vec<u8> {
    let string = string.as_ref();
    let mut result = Vec::with_capacity(string.len() * 120 / 100);

    for char in string {
        if is_unreserved_character(char) {
            result.push(*char);
        } else {
            result.push(b'%');
            result.push(hex_code(*char >> 4));
            result.push(hex_code(*char & 0xf));
        }
    }

    result
}
