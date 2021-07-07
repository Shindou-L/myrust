use std::collections::hash_map::RandomState;
use std::collections::HashMap;

use clap::{App, Arg};
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;

static DATA_PATTERN: &str = "%a, %d %b %Y %H:%M:%S GMT";

fn main() {
    let matches = App::new("cas-rust")
        .version("0.1")
        .args(&[
            Arg::from_usage("-k, --key <api-key> 'api key'").required_unless("file"),
            Arg::from_usage("-u, --user <api-user> 'api user'").required_unless("file"),
            Arg::from_usage("-f, --file [input] 'an optional input file to use'").required_unless_all(&["key","user"])
        ])
        .get_matches();

    println!("get matches successfully");
    if let Some(f) = matches.value_of("file") {
        println!("file: {}",f)
    }else {
        let key = matches.value_of("key").unwrap();
        let user = matches.value_of("user").unwrap();
        println!("api-user:{}, api-key:{}", user,key);
        let map = encrypt(user, key);
        for (k,v) in map {
            println!("{}:{}",k,v);
        }
    }
}

fn encrypt(user: &str, key: &str) -> HashMap<String, String, RandomState> {
    let now = chrono::prelude::Utc::now().format(DATA_PATTERN).to_string();
    let encrypt = hmac_sha1(key.as_bytes(),now.as_bytes());
    let mut authorization_str = String::from(user) + ":" + encrypt.as_str();
    authorization_str = base64::encode(authorization_str.as_bytes());

    let tuples = vec![(String::from("Date"),now),(String::from("Authorization"),"Basic: ".to_owned() + &authorization_str)];
    let headers = tuples.into_iter().collect();
    // let mut headers = HashMap::new();
    // headers.insert(String::from("Date"),now);
    // headers.insert(String::from("Authorization"),"Basic: ".to_owned() + &authorization_str);
    return headers;
}

fn hmac_sha1(key: &[u8],value: &[u8]) -> String {
    let mut hmac1 = Hmac::new(Sha1::new(), key);
    hmac1.input(value);
    base64::encode(hmac1.result().code())
}