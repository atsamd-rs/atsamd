#![no_std]
#![no_main]

pub fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), &str> {
    let hex = hex.trim_start_matches('#');

    let result: Result<(u8, u8, u8), &str> = if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        Ok((r, g, b))
    } else if hex.len() == 3 {
        let r = u8::from_str_radix(&hex[0..1], 16).unwrap();
        let g = u8::from_str_radix(&hex[1..2], 16).unwrap();
        let b = u8::from_str_radix(&hex[2..3], 16).unwrap();
        Ok((r * 16 + r, g * 16 + g, b * 16 + b))
    } else {
        Err("Invalid color")
    };

    result
}

#[cfg(test)]
mod tests {
    use super::hex_to_rgb;

    #[test]
    fn test_valid_hex_to_rgb() {
        let success_cases: [(&str, (u8, u8, u8)); 7] = [
            ("#000000", (0, 0, 0)),
            ("#ffffff", (255, 255, 255)),
            ("#FF0000", (255, 0, 0)),
            ("#00FF00", (0, 255, 0)),
            ("#0000FF", (0, 0, 255)),
            ("#F0F0F0", (240, 240, 240)),
            ("#abcdef", (171, 205, 239)),
        ];

        for case in success_cases.iter() {
            let (hex, expected) = case;
            let result = hex_to_rgb(hex);

            assert_eq!(result.ok().unwrap(), *expected, "hex: {}", hex);
        }
    }

    #[test]
    fn test_invalid_hex_to_rgb() {
        let invalid_result = hex_to_rgb("invalid");
        assert_eq!(invalid_result.err().unwrap(), "Invalid color");
    }
}