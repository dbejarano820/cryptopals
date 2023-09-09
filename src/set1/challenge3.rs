pub fn score_text(text: &str) -> i32 {
    let common_chars = "ETAOIN SHRDLU";
    text.to_uppercase()
        .chars()
        .filter(|&c| common_chars.contains(c))
        .count() as i32
}

pub fn single_byte_xor(input: &[u8], key: u8) -> Vec<u8> {
    input.iter().map(|&byte| byte ^ key).collect()
}

pub fn decrypt_and_score(text: &str) -> Result<String, &'static str> {
    let bytes = match hex::decode(text) {
        Ok(b) => b,
        Err(_) => return Err("Invalid Hex String"),
    };

    let mut best_score = 0;
    let mut best_string = String::new();
    let mut best_key = 0;

    for key in 0..=255 {
        let decrypted = single_byte_xor(&bytes, key);
        let decrypted_string = match String::from_utf8(decrypted) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let score = score_text(&decrypted_string);

        if score > best_score {
            best_score = score;
            best_string = decrypted_string;
            best_key = key;
        }
    }

    Ok(format!("{}: {}", best_key, best_string))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt_and_score() {
        let hex_string = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

        match decrypt_and_score(hex_string) {
            Ok(result) => {
                assert!(result.contains("Cooking MC's like a pound of bacon"));
            }
            Err(e) => panic!("An error occurred: {}", e),
        }
    }
}
