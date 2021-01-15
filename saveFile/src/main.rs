/*
 * @Author: your name
 * @Date: 2020-12-28 17:53:16
 * @LastEditTime: 2021-01-15 15:36:40
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \chrome_extension\saveFile\src\main.rs
 */

extern crate winreg;
use winreg::enums::*;
use winreg::RegKey;


// extern crate chrono ;
// use chrono::prelude::*;

// use std::time::SystemTime;


extern crate serde_json;
use serde_json::Value;


use std::io;
use std::fs::File;
// use std::io::prelude::*;
// use byteorder::{NativeEndian};
// use std::result::Result as StdResult;
// use std::error::Error as StdError;
// use std::result::Result ;
// use std::error;
// use std::str;

// use std::io::{self, Read, Write};

// use std::path::Path;

///用来识别信息分流
#[allow(dead_code)]
enum MessageIdentify{
    ChromeInstall(bool),
    FirefoxInstall(bool),
}

fn main()  {

        let  instream = io::stdin();
        let mut input = String::new();
        loop{
            match instream.read_line(&mut input) {
                 Ok(n) => {
                    println!("{} bytes read", n);
                    println!("{}", input);
                }
                Err(error) => println!("error: {}", error),
            }
        }
}


/* utils 函数 */
#[allow(dead_code)]
fn chrome_native_config(){
    let os = jugment_os();
    if os == "windows"   {
        let hklm = RegKey::predef(HKEY_CURRENT_USER);
        let (key,disp) = hklm.create_subkey("SOFTWARE\\Google\\Chrome\\NativeMessagingHosts\\chrome_nativeMessaging").unwrap();

        match disp {
            REG_CREATED_NEW_KEY => println!("A new key has been created"),
            REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
        }
        let value:String = key.get_value("").unwrap();
        if value != "D:\\threeday\\chrome_nativeMessaging.json".to_string() {
            key.set_value("", &"D:\\threeday\\chrome_nativeMessaging.json").unwrap();
        }
        let f = File::create("D:\\threeday\\chrome_nativeMessaging.json").unwrap();
        let config_str=r#"
        {
        "name":"chrome_nativeMessaging",
        "description":"Chrome native messageing api example",
        "path":"D:\\threeday\\TestWeb.exe",
        "type":"stdio",
        "allowed_origins":[
            "chrome-extension://fkdghlklbgmkokmgnoanmkedekgkckkp/"
        ]
        }"#;
        let v:Value =serde_json::from_str(config_str).unwrap();
        serde_json::to_writer(&f, &v).expect("write json failed");
    }
    else if os == "linux" {
        let f = File::create("~/.config/google-chrome/NativeMessingHosts/chrome_nativeMessaging.json").unwrap();
        let config_str=r#"
        {
        "name":"chrome_nativeMessaging",
        "description":"Chrome native messageing api example",
        "path":"~/.config/google-chrome/NativeMessingHosts/TestWeb.exe",
        "type":"stdio",
        "allowed_origins":[
            "chrome-extension://fkdghlklbgmkokmgnoanmkedekgkckkp/"
        ]
        }"#;
        let v:Value =serde_json::from_str(config_str).unwrap();
        serde_json::to_writer(&f, &v).expect("write json failed");
    }
}

/* utils 函数 */
#[allow(dead_code)]
fn firefox_native_config(){
    let os = jugment_os();
    if os == "windows"   {
        let hklm = RegKey::predef(HKEY_CURRENT_USER);
        let (_key,disp) = hklm.create_subkey("SOFTWARE\\Mozilla\\NativeMessagingHosts\\com.my_application").unwrap();

        match disp {
            REG_CREATED_NEW_KEY => println!("A new key has been created"),
            REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
        }
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let (_key2,disp2) = hklm.create_subkey("SOFTWARE\\Mozilla\\NativeMessagingHosts\\com.my_application").unwrap();
        match disp2 {
            REG_CREATED_NEW_KEY => println!("A new key has been created"),
            REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
        }
        let f = File::create("D:\\threeday\\firefox_nativeMessaging.json").unwrap();
        let config_str=r#"
        {
        "name":"firefox_nativeMessaging",
        "description":"Firefox native messageing api example",
        "path":"D:\\threeday\\TestWeb.exe",
        "type":"stdio",
        "allowed_extensions":["threeday@wingdust.com"]
        }"#;
        let v:Value =serde_json::from_str(config_str).unwrap();
        serde_json::to_writer(&f, &v).expect("write json failed");
    }
    else if os == "linux" {
        let f = File::create("~/.mozilla/native-messaging-hosts/firefox_nativeMessaging.json").unwrap();
        let config_str=r#"
        {
        "name":"firefox_nativeMessaging",
        "description":"Firefox native messageing api example",
        "path":"~/.mozilla/native-messaging-hosts/TestWeb.exe",
        "type":"stdio",
        "allowed_extensions":["threeday@wingdust.com"]
        }"#;
        let v:Value =serde_json::from_str(config_str).unwrap();
        serde_json::to_writer(&f, &v).expect("write json failed");
    }
}



/* utils 函数 */
#[allow(dead_code)]
fn write_json(message:&str,path:&str){
    let  f =File::create(format!("./{}/backup-1.json",path)).expect("create backup-1 failed");
    let v:Value=serde_json::from_str(message).expect("trans json error");
    serde_json::to_writer(&f, &v).expect("write json failed");
}


/// 判断当前运行环境的操作系统
#[allow(dead_code)]
fn jugment_os()->&'static str{
    if cfg!(target_os = "windows"){
        println!("windows");
        "windows"
    }
    else if cfg!(target_os = "linux"){
        // println!("linux");
        "linux"
    }
    else{
        "non-windows or linux"
    }
}