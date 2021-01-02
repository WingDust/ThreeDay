/*
 * @Author: your name
 * @Date: 2020-12-28 17:53:16
 * @LastEditTime: 2021-01-02 22:18:12
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \chrome_extension\saveFile\src\main.rs
 */

extern crate winreg;
use winreg::enums::*;
use winreg::RegKey;

use std::io;
use std::fs::File;
use std::io::prelude::*;
// use std::io::{self, Read, Write};

// use std::path::Path;

// HKEY_CURRENT_USER\SOFTWARE\Google\Chrome\NativeMessagingHosts\com.my_company.my_application

fn main() -> io::Result<()>  {
    if "windows" == jugment_os() {
        let hklm = RegKey::predef(HKEY_CURRENT_USER);
        let (key,disp) = hklm.create_subkey("SOFTWARE\\Google\\Chrome\\NativeMessagingHosts\\com.my_company.my_application")?;

        match disp {
            REG_CREATED_NEW_KEY => println!("A new key has been created"),
            REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
        }
        let value:String = key.get_value("")?;
        if value != "C:\\Users\\Administrator\\Desktop\\chrome_extension\\com.google.chrome.demo-win.json".to_string() {
            key.set_value("", &"C:\\Users\\Administrator\\Desktop\\chrome_extension\\com.google.chrome.demo-win.json")?;
        }
        let mut f = File::create("backup.json")?;
        f.write_all(b"https://www.baidu.com")?;//需要引入 use std::io::prelude::*;
        f.sync_data()?;

        let mut instream = io::stdin();
        let mut len = [0;4];
        instream.read(&mut len)?;
        let mut buffer = vec![0;u32::from_ne_bytes(len) as usize];
        instream.read_exact( &mut buffer)?;

    }

    
    let _a:Option<i32> = Some(3);

    let mut _s = "Hello".to_string(); 
    let _string = "Hello there.";
    let s = String::from("asdasd");
    let v = String::from("asdasd");
    let _s2 = s + &v;
    

    Ok(())
}

#[warn(dead_code)]
// 判断当前运行环境的操作系统
fn jugment_os()->String{
    if cfg!(target_os = "windows"){
        println!("windows");
        (*"windows").to_string()
    }
    else if cfg!(target_os = "linux"){
        // println!("linux");
        (*"linux").to_string()
    }
    else{
        (*"non-windows or linux").to_string()
    }
}