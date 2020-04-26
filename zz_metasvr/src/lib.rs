// extern crate redis;
// extern crate postgres;
// extern crate actix_web;
// extern crate serde_derive;
// extern crate toml;
// extern crate log;
extern crate chrono;

use std::sync::Arc;
use std::sync::Mutex;
// use std::boxed::Box;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;
// use std::ops::DerefMut;

use chrono::prelude::Local;

use deadpool::managed::{Timeouts};

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder, Error};
use deadpool_postgres::ClientWrapper;
use deadpool_redis::ConnectionWrapper;

use serde::{Serialize, Deserialize};
use futures::future::{ready, Ready};

use regex::Regex;
use std::collections::HashMap;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

// use chashmap::CHashMap;
// use concache::crossbeam::Map;

// debug < info < warn/trace < error
use log::{info, error};

include!("handle/test_handler.rs");
include!("handle/account_handler.rs");
include!("global/global_config.rs");
include!("global/global_pool.rs");
include!("proto/global_types.rs");
include!("dao/dao_acc.rs");
include!("utils/crypto_util.rs");
include!("eventbus/event_bus.rs");


#[cfg(test)]
mod tests {

    include!("utils/crypto_util.rs");
    
    // use serde::{Serialize, Deserialize};
    // use actix_web::{HttpRequest, HttpResponse, Responder, Error};
    // use futures::future::{ready, Ready};

    #[test]
    fn crypto_test() {
        let message = "Hello World!";

        let key: [u8; 32] = [65; 32];
        let iv: [u8; 16] = [97; 16];
        
        let encrypted_data = aes_wrapper::encrypt(message.as_bytes(), &key, &iv).ok().unwrap();
        // println!("encrypt:{}", String::from_utf8(encrypted_data).expect("Found invalid UTF-8"));
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
