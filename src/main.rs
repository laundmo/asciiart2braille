use core::str;
use std::io::{Read, Write};
const DOTS: [&str; 128] = parse_file();

const fn parse_file() -> [&'static str; 128] {
    let mut s = include_str!("../2dots.txt").as_bytes();
    let mut arr: [&str; 128] = [""; 128];

    while !s.is_empty() {
        let mut ascii_ord: u8 = u8::MAX;
        let mut braille_char: &str = "";
        while !s.is_empty() {
            // leading_ones is how many bytes are in utf-8
            let mut charend_i = s[0].leading_ones() as usize;
            // if its 0, we still want the current byte
            if charend_i == 0 {
                charend_i = 1;
            }
            let ch;
            // split the &[u8] buffer after the correct bytes
            (ch, s) = s.split_at(charend_i);

            // 10 = newline
            if ch.len() == 1 && ch[0] == 10 {
                break;
            }
            // store the integer of the first character or the braille &str of the second
            if ascii_ord == u8::MAX {
                ascii_ord = ch[0];
            } else if braille_char.is_empty() {
                // SAFETY:  the slice is originally from a &str and we correctly checked for the amount of bytes earlier using .leading_ones() on the first byte
                braille_char = unsafe { str::from_utf8_unchecked(ch) };
            }
        }
        // store the braille character in the array with index of the ascii characters byte as integer
        // allows us to do arr["a".as_bytes()[0] as usize] or equivalent lookup later
        arr[ascii_ord as usize] = braille_char;
    }

    arr
}

fn main() {
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    for b in stdin.bytes().flatten() {
        let res = DOTS[b as usize];
        if res.is_empty() {
            stdout.write_all(&[b]).unwrap();
        } else {
            stdout.write_all(res.as_bytes()).unwrap();
        }
    }
}
