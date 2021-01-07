/*
 * @Author: your name
 * @Date: 2021-01-04 13:05:37
 * @LastEditTime: 2021-01-07 21:53:28
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

fn main() {
    loop {
        let message = match protocol::read_stdin() {
            Ok(m) => m,
            Err(_) => process::exit(1),
        };
        if message != "" {
            if Path::new("backup-1.json").exists() {
              if  is_empty() {
                File::create("backup-1.json").expect("backup-1.json write error");
              }
              else{//文件内容不为空
                if is_in_threedays() {

                }
                  /* 1. 先知道文件最后一次的写入时间
                     2. 当为明天时就新建一个文件写入
                     3. 写入的长度大于
                  */

              }
            }
        }

        println_stderr!("received {}", message);
        protocol::write_stdout(message);
    }
}


////////////////////////////
/// 暂只用知道是否是空内容文件
///
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

fn is_in_threedays() ->bool{
    let metadata = fs::metadata("backup-1.json").expect("fileinfo error");
    let time = metadata.modified().expect("time error");
    let msecond : DateTime<Local>=time.into();
    let m =msecond.date().and_hms(0, 0, 0).timestamp();

    let dt = Local::now();
    let s1 = dt.date().and_hms(0, 0, 0).timestamp();
    if s1-m == 24*3600 {
        return true
    }else{
        return false
    }
}