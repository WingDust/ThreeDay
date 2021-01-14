/*
 * @Author: your name
 * @Date: 2021-01-04 13:05:37
 * @LastEditTime: 2021-01-14 10:57:49
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

#[allow(dead_code)]
struct Backup{
   pub backup1:bool,
   pub backup2:bool,
   pub backup3:bool,
   nums:i32,
}

fn main() {
    loop {
        let message = match protocol::read_stdin() {
            Ok(m) => m,
            Err(_) => process::exit(1),
        };
        // println_stderr!("{}",message != "");
                  /* 1. 先知道文件最后一次的写入时间
                     2. 当为明天时就新建一个文件写入
                     3. 写入的长度大于
                  */
                  /* 1. 先判断是那个浏览器传入了数据，2. 判断操作系统 */
        if message != "" {
            // println_stderr!("1. {}",message != "");
            if backup.backup1 {
                    println_stderr!("2. ");
                if is_in_threedays() {
                    println_stderr!("3. ");
                    // println_stderr!("backup.nums ==1: {}",backup.nums ==1);
                    if backup.nums ==2 { // 表示 1、2 存在
                        fs::rename("backup-2.json", "backup-3.json").expect("rename 2-3 failed");
                        fs::rename("backup-1.json", "backup-2.json").expect("rename 1-2 failed");
                        write_json(&message);
                        println_stderr!("received 2 {}", message);
                    }
                    else if backup.nums == 3 {// 表示 1、2、3 存在
                        fs::remove_file("backup-3.json").expect("remove backup-3 failed");
                        fs::rename("backup-2.json", "backup-3.json").expect("rename 2-3 failed");
                        fs::rename("backup-1.json", "backup-2.json").expect("rename 1-2 failed");
                        write_json(&message);
                        // f.write_all(message.as_bytes()).expect("write file failed");
                        println_stderr!("received 3 {}", message);
                    }
                    else{// 表示 1 存在
                        fs::rename("backup-1.json", "backup-2.json").expect("rename 1-2 failed");
                        write_json(&message);
                        println_stderr!("received 1 {}", message);
                    }
                }
                else{
                    write_json(&message);
                    println_stderr!("received 4 {}", message);
                }
            }
            else{
                // f.write_all(message.as_bytes()).expect("write file failed");
                write_json(&message);
                println_stderr!("received 4 {}", message);
            }
        }

        println_stderr!("received {}", message);
        protocol::write_stdout(message);
    }
}


////////////////////////////
/// 暂只用知道是否是空内容文件
///
#[allow(dead_code)]
fn is_empty()-> bool{
    let contents = fs::read_to_string("backup-1.json").expect(" Something went woring reading the file");
    if contents == "" {
        return true
    }
    else{
        return false
    }
    // let v:Value = serde_json::from_str(&contents).expect("backup.json havs syntax problem");
}
//////////////////////////////
///判断文件最后一次修改时间距今是否至少有一天的时间
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

#[allow(dead_code)]
fn exists(browser:&str)->i32{
    if Path::new(&format!("./{}/backup-2.json",browser)).exists() {return 2} 
    else if Path::new(&format!("./{}/backup-3.json",browser)).exists() {return 3} 
    else { return Default::default()}
}

#[allow(dead_code)]
fn write_json(message:&str,path:&str){
    let  f =File::create("backup-1.json").expect("create backup-1 failed");
    let v:Value=serde_json::from_str(message).expect("trans json error");
    serde_json::to_writer(&f, &v).expect("write json failed");
}

// 判断当前运行环境的操作系统
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

#[allow(dead_code)]
fn chrome_native_config(){
    let os = jugment_os();
    if os == "windows"   {
        let hklm = RegKey::predef(HKEY_CURRENT_USER);
        let (key,disp) = hklm.create_subkey("SOFTWARE\\Google\\Chrome\\NativeMessagingHosts\\com.my_company.my_application").unwrap();

        match disp {
            REG_CREATED_NEW_KEY => println!("A new key has been created"),
            REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
        }
        let value:String = key.get_value("").unwrap();
        if value != "C:\\Users\\Administrator\\Desktop\\chrome_extension\\com.google.chrome.demo-win.json".to_string() {
            key.set_value("", &"C:\\Users\\Administrator\\Desktop\\chrome_extension\\com.google.chrome.demo-win.json").unwrap();
        }
    }
    else if os == "linux" {

    }
}

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
    }
    else if os == "linux" {

    }
}

#[allow(dead_code)]
fn jugment_browser(message:&str){
    let v:Value=serde_json::from_str(message).expect("trans json error");
    let first_v=v.get(0).unwrap();
    let second_v=v.get(1).unwrap();
    if  first_v==1{                 //表明这个消息是来自 Chrome  Installed消息
        chrome_native_config();
    }
    else if second_v == 1{          //表明这个消息是来自 FireFox Installed消息
        firefox_native_config();
    }
    else if  first_v == "chrome"{   //表明这个消息是来自 Chrome  Tabs消息
        let backup = Backup{
            backup1:Path::new("./chrome/backup-1.json").exists(),
            backup2:Path::new("./chrome/backup-2.json").exists(),
            backup3:Path::new("./chrome/backup-3.json").exists(),
            nums: exists("chrome"),
        };
    }
    else if first_v == "firefox"{   //表明这个消息是来自 FireFox Tabs消息
        let backup = Backup{
            backup1:Path::new("./firefox/backup-1.json").exists(),
            backup2:Path::new("./firefox/backup-2.json").exists(),
            backup3:Path::new("./firefox/backup-3.json").exists(),
            nums: exists("firefox"),
        };
    }
    else {
    }
}
#[allow(dead_code)]
fn order_jugment(backup:Backup,browser:&str){
    if backup.backup1 {
        if is_in_threedays(browser){
            if backup.nums ==2 { // 表示 1、2 存在
                fs::rename(format!("./{}/backup-2.json",browser), format!("./{}/backup-3.json",browser)).expect("rename 2-3 failed");
                fs::rename(format!("./{}/backup-1.json",browser), format!("./{}/backup-2.json",browser)).expect("rename 1-2 failed");
                // write_json(&message);
                // println_stderr!("received 2 {}", message);
            }
            else if backup.nums == 3 {// 表示 1、2、3 存在
                fs::remove_file(format!("./{}/backup-3.json",browser)).expect("remove backup-3 failed");
                fs::rename(format!("./{}/backup-2.json",browser), format!("./{}/backup-3.json",browser)).expect("rename 2-3 failed");
                fs::rename(format!("./{}/backup-1.json",browser), format!("./{}/backup-2.json",browser)).expect("rename 1-2 failed");
                // write_json(&message);
                // f.write_all(message.as_bytes()).expect("write file failed");
                // println_stderr!("received 3 {}", message);
            }
            else{// 表示 1 存在
                fs::rename(format!("./{}/backup-1.json",browser), format!("./{}/backup-2.json",browser)).expect("rename 1-2 failed");
                // write_json(&message);
                // println_stderr!("received 1 {}", message);
            }
        }
    }
    else{
        // write_json(&message,);
        // println_stderr!("received 4 {}", message);

    }

}