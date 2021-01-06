
#[macro_use(println_stderr)]
extern crate webextension_protocol as protocol;
use std::io::Write;
use std::process;

extern crate serde_json ;
use serde_json::{Value,Result};

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
            if Path::new("backup.json").exists() {
              if  is_empty() {
                File::create("backup.json").expect("backup.json write error");
              }
              else{//文件内容不为空
                let metadata = fs::metadata("backup.json").expect("asd");
                let time = metadata.modified();

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
    let contents = fs::read_to_string("backup.json").expect(" Something went woring reading the file");
    if contents == "" {
        return true
    }
    else{
        return false
    }
    // let v:Value = serde_json::from_str(&contents).expect("backup.json havs syntax problem");
}