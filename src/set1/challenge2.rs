pub fn fixed_xor(value1: &[u8], value2: &[u8]) -> Result<Vec<u8>, &'static str> {
    if value1.len() != value2.len() {
        return Err("Inputs must have the same length");
    }

    let result: Vec<u8> = value1
        .iter()
        .zip(value2.iter())
        .map(|(a, b)| a ^ b)
        .collect();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn test_fixed_xor() {
        let hex_string = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
        let xor_with = hex::decode("686974207468652062756c6c277320657965").unwrap();
        let expected_result = hex::decode("746865206b696420646f6e277420706c6179").unwrap();

        match fixed_xor(&hex_string, &xor_with) {
            Ok(result) => assert_eq!(result, expected_result),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }
}
