extern crate rustc_serialize;

pub fn fixed_xor(first: Vec<u8>, second: Vec<u8>) -> Vec<u8> {
    assert_eq!(first.len(), second.len());
    return first.iter().zip(second).map(|(x, y)| x ^ y).collect(); 
}

#[cfg(test)]
mod tests {
    use one::base_64_utils::rustc_serialize::hex::{FromHex, ToHex};
    use super::*;

    #[test]
    fn test_all() {
        let first = "1c0111001f010100061a024b53535009181c".from_hex().unwrap();
        let second = "686974207468652062756c6c277320657965".from_hex().unwrap();
        let result = fixed_xor(first, second).to_hex();
        assert!(result == "746865206b696420646f6e277420706c6179");
    }
}

