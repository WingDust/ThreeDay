/*
 * @Author: your name
 * @Date: 2020-12-28 17:53:16
 * @LastEditTime: 2021-01-12 22:23:24
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \chrome_extension\saveFile\src\main.rs
 */

extern crate winreg;
use winreg::enums::*;
use winreg::RegKey;


extern crate chrono ;
use chrono::prelude::*;

use std::time::SystemTime;

#[macro_use(println_stderr)]
extern crate webextension_protocol as protocol;
use std::io::Write;
use std::process;


// extern crate chrome_native_messaging;
// use chrome_native_messaging::event_loop;
// use chrome_native_messaging::read_input;

extern crate serde_json;
use serde_json::Value;


use std::io;
use std::fs;
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

    //"path":"C:\\Users\\Administrator\\Desktop\\chrome_extension\\saveFile\\target\\debug\\saveFile.exe",
        // let mut f = File::create("backup.json")?;
        // f.write_all(b"https://www.baidu.com")?;//需要引入 use std::io::prelude::*;
        // f.sync_data()?;

        // let mut instream = io::stdin();
        // let mut len = [0;4];//这个指初始化长度为4每一个为0（间接也定义了数组类型）的数组
        // instream.read(&mut len)?;
        // let mut buffer = vec![0;u32::from_ne_bytes(len) as usize];
        // instream.read_exact( &mut buffer)?;

        // println!("{:?}",str::from_utf8(&buffer));

        let metadata = fs::metadata("backup.json").expect("asd");
        let time = metadata.modified().expect("time error");
        // let t: DateTime<Utc> = time.into();
        let t: DateTime<Local> = time.into();
        let t1 =t.date();
        let dt2: DateTime<Local> = Local.timestamp(0, 0);
        println!("{}",t1.and_hms(0,0,0).format("%d/%m/%Y %T"));
        println!("{}",t1.format("%d/%m/%Y"));
        println!("{}",t.format("%d/%m/%Y %T"));
        println!("{}",dt2.format("%d/%m/%Y %T"));

        let system_time = SystemTime::now();
        let datetime: DateTime<Utc> = system_time.into();
        println!("{}", datetime.format("%d/%m/%Y %T"));
        let dt = Local::now();
        println!("dt: {}", dt);
        println!("dt: {}", dt.timestamp_millis());

        let s1 = dt.date().and_hms(0, 0, 0).timestamp();
        let s2 = t.date().and_hms(0, 0, 0).timestamp();

        println!("s1: {}", s1);
        println!("s2: {}", s2);


        let f =File::create("backup-1.json").expect("create backup-1 failed");
        // f.write_all(message.as_bytes()).expect("write file failed");
        let v:Value=serde_json::from_str("[1,2,3]").expect("trans json error");
        // let v1 =serde_json::to_string(&v).unwrap();
        serde_json::to_writer(&f, &v).expect("write json failed");




        loop {
            let message = match protocol::read_stdin() {
                Ok(m) => m,
                Err(_) => process::exit(1),
            };
            println_stderr!("received {}", message);
            protocol::write_stdout(message);
        }


        
        // instream.read_u32::<NativeEndian>();
        // let  instream = io::stdin();
        // let mut input = String::new();
        // loop{
        //     match instream.read_line(&mut input) {
        //          Ok(n) => {
        //             println!("{} bytes read", n);
        //             println!("{}", input);
        //         }
        //         Err(error) => println!("error: {}", error),
        //     }
        // }

        // event_loop(test)


    }

    
    // let _a:Option<i32> = Some(3);

    // let mut _s = "Hello".to_string(); 
    // let _string = "Hello there.";
    // let s = String::from("asdasd");
    // let v = String::from("asdasd");
    // let _s2 = s + &v;
    

    Ok(())
}



// fn test(val:serde_json::Value)->Result<(),error::Error>{

//     Ok(())

// }



#[warn(dead_code)]
// 判断当前运行环境的操作系统
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