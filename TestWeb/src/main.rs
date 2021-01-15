/*
 * @Author: your name
 * @Date: 2021-01-04 13:05:37
 * @LastEditTime: 2021-01-15 11:14:00
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \chrome_extension\TestWeb\src\main.rs
 */

#[macro_use(println_stderr)]
extern crate webextension_protocol as protocol;
use std::io::Write;
use std::process;

extern crate serde_json ;
use serde_json::{Value};

extern crate chrono;
use chrono::prelude::*;


extern crate winreg;
use winreg::enums::*;
use winreg::RegKey;

use std::fs;
use std::fs::File;
use std::path::Path;

///用来判断保存了几次状态
#[allow(dead_code)]
struct Backup{
   pub backup1:bool,
   pub backup2:bool,
   pub backup3:bool,
   nums:i32,
}

///用来识别信息分流
#[allow(dead_code)]
enum MessageIdentify{
    ChromeInstall(bool),
    ChromeTabs(bool),
    FirefoxInstall(bool),
    FirefoxTabs(bool)
}


fn main() {
    loop {
        let message = match protocol::read_stdin() {
            Ok(m) => m,
            Err(_) => process::exit(1),
        };
                  /* 1. 先知道文件最后一次的写入时间
                     2. 当为明天时就新建一个文件写入
                     3. 写入的长度大于
                  */
                  /* 1. 先判断是那个浏览器传入了数据，2. 判断操作系统 */
        runtime(&message);

        println_stderr!("received {}", message);
        protocol::write_stdout(message);
    }
}

/* run 函数 */
/// 程序的运行函数
#[allow(dead_code)]
fn runtime(message:&str){
    let message_result:MessageIdentify = jugment_browser_message(message).unwrap();
    match  message_result {
        MessageIdentify::ChromeInstall(true)=>{chrome_native_config();},
        MessageIdentify::FirefoxInstall(true)=>{firefox_native_config();}, 
        MessageIdentify::ChromeTabs(true)=>{order_jugment( "chrome", message);},
        MessageIdentify::FirefoxTabs(true)=>{order_jugment( "firefox", message);},
        _=>{}
    }

}

/* utils 函数 */
/// 对浏览器传入的信息进行作用辨识
#[allow(dead_code)]
fn jugment_browser_message(message:&str) ->Option<MessageIdentify> {
    let v:Value=serde_json::from_str(message).expect("trans json error");
    let first_v=v.get(0).unwrap();
    let second_v=v.get(1).unwrap();
    if  first_v==1{                 //表明这个消息是来自 Chrome  Installed消息
        Some(MessageIdentify::ChromeInstall(true))
    }
    else if second_v == 1{          //表明这个消息是来自 FireFox Installed消息
        Some(MessageIdentify::FirefoxInstall(true)) 
    }
    else if  first_v == "chrome"{   //表明这个消息是来自 Chrome  Tabs消息
        Some(MessageIdentify::ChromeTabs(true))
    }
    else if first_v == "firefox"{   //表明这个消息是来自 FireFox Tabs消息
        Some(MessageIdentify::FirefoxTabs(true))
    }
    else {None}
}

/* utils 函数 */
/// 对保存的
#[allow(dead_code)]
fn order_jugment(browser:&str,message:&str){
    let backup = Backup{
        backup1:Path::new(&format!("./{}/backup-1.json",browser)).exists(),
        backup2:Path::new(&format!("./{}/backup-2.json",browser)).exists(),
        backup3:Path::new(&format!("./{}/backup-3.json",browser)).exists(),
        nums: exists(browser),
    };
    if backup.backup1 {
        if is_in_threedays(browser){
            if backup.nums == 2 { // 表示 1、2 存在
                fs::rename(format!("./{}/backup-2.json",browser), format!("./{}/backup-3.json",browser)).expect("rename 2-3 failed");
                fs::rename(format!("./{}/backup-1.json",browser), format!("./{}/backup-2.json",browser)).expect("rename 1-2 failed");
                write_json(message,browser);
                // println_stderr!("received 2 {}", message);
            }
            else if backup.nums == 3 {// 表示 1、2、3 存在
                fs::remove_file(format!("./{}/backup-3.json",browser)).expect("remove backup-3 failed");
                fs::rename(format!("./{}/backup-2.json",browser), format!("./{}/backup-3.json",browser)).expect("rename 2-3 failed");
                fs::rename(format!("./{}/backup-1.json",browser), format!("./{}/backup-2.json",browser)).expect("rename 1-2 failed");
                write_json(message,browser);
                // f.write_all(message.as_bytes()).expect("write file failed");
                // println_stderr!("received 3 {}", message);
            }
            else{// 表示 1 存在
                fs::rename(format!("./{}/backup-1.json",browser), format!("./{}/backup-2.json",browser)).expect("rename 1-2 failed");
                write_json(message,browser);
                // println_stderr!("received 1 {}", message);
            }
        }
    }
    else{
        write_json(message,browser);
        // write_json(&message,);
        // println_stderr!("received 4 {}", message);

    }

}


/* utils 函数 */
#[allow(dead_code)]
fn exists(browser:&str)->i32{
    if Path::new(&format!("./{}/backup-2.json",browser)).exists() {return 2} 
    else if Path::new(&format!("./{}/backup-3.json",browser)).exists() {return 3} 
    else { return Default::default()}
}

/* utils 函数 */
#[allow(dead_code)]
fn write_json(message:&str,path:&str){
    let  f =File::create(format!("./{}/backup-1.json",path)).expect("create backup-1 failed");
    let v:Value=serde_json::from_str(message).expect("trans json error");
    serde_json::to_writer(&f, &v).expect("write json failed");
}

/* utils 函数 */
/// 判断文件最后一次修改时间距今是否至少有一天的时间
/// true:有一天 false:没有一天
fn is_in_threedays(browser:&str) ->bool{
    let metadata = fs::metadata(format!("./{}/backup-1.json",browser)).expect("fileinfo error");
    let time = metadata.modified().expect("time error");
    let msecond : DateTime<Local>=time.into();
    let m =msecond.date().and_hms(0, 0, 0).timestamp();

    let dt = Local::now();
    let s1 = dt.date().and_hms(0, 0, 0).timestamp();
    if s1-m >= 24*3600 {
        return true
    }else{
        return false
    }
}

/* utils 函数 */
/// 判断当前运行环境的操作系统
#[allow(dead_code)]
fn jugment_os()->&'static str{
    if cfg!(target_os = "windows"){
        "windows"
    }
    else if cfg!(target_os = "linux"){
        "linux"
    }
    else{
        "non-windows or linux"
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
/// 暂只用知道是否是空内容文件
#[allow(dead_code)]
fn is_empty()-> bool{
    let contents = fs::read_to_string("backup-1.json").expect(" Something went woring reading the file");
    if contents == "" {
        return true
    }
    else{
        return false
    }
}