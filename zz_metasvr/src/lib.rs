extern crate chrono;

use std::env;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::io::prelude::*;

use serde::{Serialize, Deserialize};

use log::{info, error};

include!("handle/test_handler.rs");
include!("global/global_config.rs");
include!("proto/global_types.rs");

#[cfg(test)]
mod tests {

    include!("utils/crypto_util.rs");
    
    #[test]
    fn crypto_test() {
        let message = "Hello World!";

        let key: [u8; 32] = [65; 32];
        let iv: [u8; 16] = [97; 16];
        
        let encrypted_data = aes_wrapper::encrypt(message.as_bytes(), &key, &iv).ok().unwrap();
        let decrypted_data = aes_wrapper::decrypt(&encrypted_data[..], &key, &iv).ok().unwrap();
    
        assert!(message.as_bytes() == &decrypted_data[..]);
    }

    #[test]
    fn regexp_test() {
        let mail_regexp = regex::Regex::new(r"^[.a-zA-Z0-9_-]+@[a-zA-Z0-9_-]+(\.[a-zA-Z0-9_-]+)+$").unwrap();
        assert!(mail_regexp.is_match("ccNB-_a123@gg.cn"));
        assert!(mail_regexp.is_match("ccNB-_a.123@gg.com.cn"));
        assert!(!mail_regexp.is_match("ccNB-_a.123@.com.cn"));

        let mphone_regexp = regex::Regex::new(r"^((\+86)|(86))?(1[3|5|7|8|9])\d{9}$").unwrap();
        assert!(mphone_regexp.is_match("13888888888"));
        assert!(mphone_regexp.is_match("19888888888"));
        assert!(!mphone_regexp.is_match("198888888888"));
        assert!(!mphone_regexp.is_match("19888a88888"));
    }
}
