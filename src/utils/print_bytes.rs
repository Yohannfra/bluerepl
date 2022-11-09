use rhexdump;

use std::str;

pub fn bytes_to_str(bytes: &Vec<u8>, format: &str) -> String {
    let mut out: String = String::new();

    match format {
        "bin" => {
            out.push('[');
            for (idx, b) in bytes.iter().enumerate() {
                out.push_str(&format!(
                    "{:#b}{}",
                    b,
                    if idx == bytes.len() - 1 { "" } else { ", " }
                ));
            }
            out.push(']');
        }
        "hex" => {
            out.push('[');
            for (idx, b) in bytes.iter().enumerate() {
                out.push_str(&format!(
                    "0x{:02x}{}",
                    b,
                    if idx == bytes.len() - 1 { "" } else { ", " }
                ));
            }
            out.push(']');
        }
        "dec" => out.push_str(&format!("{:?}", bytes)),
        "text" => out.push_str(str::from_utf8(bytes).unwrap_or("Invalid string")),
        "hexdump" => out.push_str(&rhexdump::hexdump(bytes)),
        _ => panic!("Unknown format: '{}'", format),
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_str() {
        let bytes: Vec<u8> = vec![1, 2, 3];
        assert_eq!(bytes_to_str(&bytes, "hex"), "[0x01, 0x02, 0x03]".to_owned());
        assert_eq!(bytes_to_str(&bytes, "dec"), "[1, 2, 3]".to_owned());
        assert_eq!(bytes_to_str(&bytes, "bin"), "[0b1, 0b10, 0b11]".to_owned());
        assert_eq!(
            bytes_to_str(&bytes, "hexdump"),
            "00000000: 01 02 03 | ...".to_owned()
        );

        let bytes: Vec<u8> = vec![104, 101, 108, 108, 111]; // hello
        assert_eq!(
            bytes_to_str(&bytes, "hex"),
            "[0x68, 0x65, 0x6c, 0x6c, 0x6f]".to_owned()
        );
        assert_eq!(
            bytes_to_str(&bytes, "dec"),
            "[104, 101, 108, 108, 111]".to_owned()
        );
        assert_eq!(
            bytes_to_str(&bytes, "bin"),
            "[0b1101000, 0b1100101, 0b1101100, 0b1101100, 0b1101111]".to_owned()
        );
        assert_eq!(
            bytes_to_str(&bytes, "hexdump"),
            "00000000: 68 65 6c 6c 6f | hello".to_owned()
        );
        assert_eq!(bytes_to_str(&bytes, "text"), "hello".to_owned());
    }
}
