extern crate rustc_serialize;

use self::rustc_serialize::base64;
use self::rustc_serialize::base64::{ToBase64};

pub fn bytes_to_base_64(bytes: Vec<u8>) -> String {
    return bytes.to_base64(base64::STANDARD);
}

#[cfg(test)]
mod tests {
    use one::base_64_utils::rustc_serialize::hex::FromHex;
    use super::*;

    #[test]
    fn test_all() {
        let test = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".from_hex().unwrap();
        let result = bytes_to_base_64(test);
        assert!(result == "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }
}

