/*
 * @Author: your name
 * @Date: 2021-01-04 13:05:37
 * @LastEditTime: 2021-01-08 19:18:42
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
   nums:i32,
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
                nums: exists(),
            };
            if backup.backup1 {
                if is_in_threedays() {
                    if backup.nums == 1{// 表示 1 存在
                        fs::rename("backup-1.json", "backup-2.json").expect("rename 1-2 failed");
                        let mut f =File::create("backup-1.json").expect("create backup-1 failed");
                        f.write_all(message.as_bytes()).expect("write file failed");
                    }
                    else if backup.nums ==2 { // 表示 1、2 存在
                        fs::rename("backup-2.json", "backup-3.json").expect("rename 2-3 failed");
                        fs::rename("backup-1.json", "backup-2.json").expect("rename 1-2 failed");
                        let mut f =File::create("backup-1.json").expect("create backup-1 failed");
                        f.write_all(message.as_bytes()).expect("write file failed");
                    }
                    else if backup.nums == 3 {// 表示 1、2、3 存在
                        fs::remove_file("backup-3.json").expect("remove backup-3 failed");
                        fs::rename("backup-2.json", "backup-3.json").expect("rename 2-3 failed");
                        fs::rename("backup-1.json", "backup-2.json").expect("rename 1-2 failed");
                        let mut f =File::create("backup-1.json").expect("create backup-1 failed");
                        f.write_all(message.as_bytes()).expect("write file failed");
                    }
                }
                else{
                    let mut f =File::create("backup-1.json").expect("create backup-1 failed");
                    f.write_all(message.as_bytes()).expect("write file failed");
                    println_stderr!("received {}", message);
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
/// true:有一天 false:没有一天
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
fn exists()->i32{
    if Path::new("backup-2.json").exists() {return 2} 
    else if Path::new("backup-3.json").exists() {return 3} 
    else { return Default::default()}
}