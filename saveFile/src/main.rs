/*
 * @Author: your name
 * @Date: 2020-12-28 17:53:16
 * @LastEditTime: 2021-01-21 15:08:23
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \chrome_extension\saveFile\src\main.rs
 */


#[cfg(target_os="windows")]
extern crate winreg;
#[cfg(target_os="windows")]
use winreg::enums::*;
#[cfg(target_os="windows")]
use winreg::RegKey;

// extern crate chrono ;
// use chrono::prelude::*;

// use std::time::SystemTime;


extern crate serde_json;
use serde_json::Value;


use std::fs;
use std::fs::File;
use std::path::Path;
// use std::process;

fn main()  {
    // option_to_native();
    chrome_native_config();
    firefox_native_config();
    // let  instream = io::stdin();
    // let mut input = String::new();
    // loop{
    //     match instream.read_line(&mut input) {
    //             Ok(_n) => {
    //             // println!("{} bytes read", n);
    //             println!("请输入小写 q 退出程序");
    //             if input == "q".to_string(){
    //                 println!("退出程序");
    //                 process::exit(1)
    //             }
    //             // println!("{}", input);
    //         }
    //         Err(error) => println!("error: {}", error),
    //     }
    // }
}

/* utils 函数 */
#[allow(dead_code)]
#[cfg(target_os="windows")]
fn chrome_native_config(){

    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let (key,disp) = hklm.create_subkey("SOFTWARE\\Google\\Chrome\\NativeMessagingHosts\\chrome_nativeMessaging").unwrap();

    match disp {
        REG_CREATED_NEW_KEY => println!("A new key has been created"),
        REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
    }
    key.set_value("", &"D:\\threeday\\chrome_nativeMessaging.json").unwrap();
    let value:String = key.get_value("").unwrap();
    if value != "D:\\threeday\\chrome_nativeMessaging.json".to_string() {
        key.set_value("", &"D:\\threeday\\chrome_nativeMessaging.json").unwrap();
    }

    let path = "D:\\threeday\\chrome_nativeMessaging.json";
    writefile(path);
    let f = File::open(path).unwrap();
    let config_str=r#"
    {
    "name":"com.chrome.nativemessaging",
    "description":"Chrome native messageing api example",
    "path":"D:\\threeday\\TestWeb.exe",
    "type":"stdio",
    "allowed_origins":[
        "chrome-extension://hgibimofjkchfnpmfhnigfhkkkahlmah/"
    ]
    }"#;
    let v:Value =serde_json::from_str(config_str).unwrap();
    serde_json::to_writer(&f, &v).expect("write json failed");
}

#[cfg(target_os="linux")]
fn chrome_native_config(){
    let path = "~/.config/google-chrome/NativeMessingHosts/chrome_nativeMessaging.json";
    writefile(path);
    let f = File::open(path).unwrap();
    let config_str=r#"
    {
    "name":"chrome_nativeMessaging",
    "description":"Chrome native messageing api example",
    "path":"~/.config/google-chrome/NativeMessingHosts/TestWeb",
    "type":"stdio",
    "allowed_origins":[
        "chrome-extension://fkdghlklbgmkokmgnoanmkedekgkckkp/"
    ]
    }"#;
    let v:Value =serde_json::from_str(config_str).unwrap();
    serde_json::to_writer(&f, &v).expect("write json failed");
}
/* utils 函数 */
#[allow(dead_code)]
#[cfg(target_os="windows")]
fn firefox_native_config(){
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let (key,disp) = hklm.create_subkey("SOFTWARE\\Mozilla\\NativeMessagingHosts\\com.wingdust.threeday").unwrap();

    match disp {
        REG_CREATED_NEW_KEY => println!("A new key has been created"),
        REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
    }
    key.set_value("", &"D:\\threeday\\firefox_nativeMessaging.json").unwrap();
    let value:String = key.get_value("").unwrap();
    if value != "D:\\threeday\\firefox_nativeMessaging.json".to_string() {
        key.set_value("", &"D:\\threeday\\firefox_nativeMessaging.json").unwrap();
    }
    let hklm2 = RegKey::predef(HKEY_LOCAL_MACHINE);
    let (key2,disp2) = hklm2.create_subkey("SOFTWARE\\Mozilla\\NativeMessagingHosts\\com.wingdust.threeday").unwrap();
    match disp2 {
        REG_CREATED_NEW_KEY => println!("A new key has been created"),
        REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
    }
    key2.set_value("", &"D:\\threeday\\firefox_nativeMessaging.json").unwrap();
    let value2:String = key2.get_value("").unwrap();
    if value2 != "D:\\threeday\\firefox_nativeMessaging.json".to_string() {
        key2.set_value("", &"D:\\threeday\\firefox_nativeMessaging.json").unwrap();
    }

    let path = "D:\\threeday\\firefox_nativeMessaging.json";
    writefile(path);
    let f = File::open(path).unwrap();
    let config_str=r#"
    {
    "name":"com.wingdust.threeday",
    "description":"Firefox native messageing api example",
    "path":"D:\\threeday\\TestWeb.exe",
    "type":"stdio",
    "allowed_extensions":["threeday@wingdust.com"]
    }"#;
    let v:Value =serde_json::from_str(config_str).unwrap();
    serde_json::to_writer(&f, &v).expect("write json failed");
}

#[cfg(target_os="linux")]
fn  firefox_native_config(){
    let path = "~/.mozilla/native-messaging-hosts/firefox_nativeMessaging.json";
    writefile(path);
    let f = File::open(path).unwrap();
    let config_str=r#"
    {
    "name":"firefox_nativeMessaging",
    "description":"Firefox native messageing api example",
    "path":"~/.mozilla/native-messaging-hosts/TestWeb",
    "type":"stdio",
    "allowed_extensions":["threeday@wingdust.com"]
    }"#;
    let v:Value =serde_json::from_str(config_str).unwrap();
    serde_json::to_writer(&f, &v).expect("write json failed");

}


#[allow(dead_code)]
fn writefile(path:&str){
    let ancestors = Path::new(path).ancestors();
    for i in ancestors {
        if i.is_dir(){
            fs::create_dir_all(i).unwrap();
        }
    }
    // if path.is_relative(){
    // }
}

/// 判断当前运行环境的操作系统
#[allow(dead_code)]
fn jugment_os()->&'static str{
    if cfg!(target_os = "windows"){
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


/* utils 函数 */
#[allow(dead_code)]
fn option_to_native(){
    println!("
    请通过输入序号
    1. Chrome
    2. Firefox
    ");

}
