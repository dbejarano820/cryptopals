pub fn hex_to_base64(hex: &str) -> Result<String, &'static str> {
    // Convert Hex to Bytes
    let raw_bytes = match hex::decode(hex) {
        Ok(b) => b,
        Err(_) => return Err("Invalid Hex String"),
    };

    // Encode raw bytes to Base64
    let base64_bytes = data_encoding::BASE64.encode(&raw_bytes);

    // Return as raw bytes
    Ok(base64_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_base64() {
        let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected_base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        match hex_to_base64(hex_string) {
            Ok(result) => assert_eq!(result, expected_base64),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }
}
