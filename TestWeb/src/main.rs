/*
 * @Author: your name
 * @Date: 2021-01-04 13:05:37
 * @LastEditTime: 2021-01-08 13:43:40
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \chrome_extension\TestWeb\src\main.rs
 */

#[macro_use(println_stderr)]
extern crate webextension_protocol as protocol;
use std::io::Write;
use std::process;

extern crate serde_json ;
// use serde_json::{Value,Result};

extern crate chrono;
use chrono::prelude::*;

use std::fs;
use std::fs::File;
use std::path::Path;

#[allow(dead_code)]
struct Backup{
   pub backup1:bool,
   pub backup2:bool,
   pub backup3:bool,
}

fn main() {
    loop {
        let message = match protocol::read_stdin() {
            Ok(m) => m,
            Err(_) => process::exit(1),
        };
        if message != "" {
            let backup = Backup{
                backup1:Path::new("backup-1.json").exists(),
                backup2:Path::new("backup-2.json").exists(),
                backup3:Path::new("backup-3.json").exists(),
            };
            if backup.backup1 {
                if is_in_threedays() {

                }
                else{

                }
                  /* 1. 先知道文件最后一次的写入时间
                     2. 当为明天时就新建一个文件写入
                     3. 写入的长度大于
                  */
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
fn is_in_threedays() ->bool{
    let metadata = fs::metadata("backup-1.json").expect("fileinfo error");
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
fn exists(){
    // let backup1 = Path::new("backup-1.json").exists();
    // let backup2 = Path::new("backup-2.json").exists();
    // let backup3 = Path::new("backup-3.json").exists();
}