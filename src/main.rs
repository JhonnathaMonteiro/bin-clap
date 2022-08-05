use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("expected file path!");
    let bytes_content = fs::read(file_path).expect("Something whent wrong reading file bytes");

    bytes_content.chunks(16).enumerate().for_each(|(i, inner)| {
        let mut middle_content = String::new();
        let mut perusal = String::from("|");

        let mut right_offset = 49 - (3 * inner.len());
        if inner.len() >= 8 {
            right_offset -= 1
        }
        for (j, c) in inner.into_iter().enumerate() {
            middle_content.push_str(&format!("{:02x} ", *c as u8));
            if j == 7 {
                middle_content.push_str(" ");
            }

            if *c >= 32 && *c < 127 {
                perusal.push(*c as char)
            } else {
                perusal.push_str(".");
            }
        }

        for _ in 0..right_offset {
            middle_content.push_str(" ");
        }

        perusal.push_str("|");
        println!("{:08x} {} {perusal}", i * 16, middle_content);
    });
}
