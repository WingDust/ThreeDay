/*
 * @Author: your name
 * @Date: 2020-12-28 17:53:16
 * @LastEditTime: 2020-12-30 10:52:18
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \chrome_extension\saveFile\src\main.rs
 */

extern crate winreg;
use winreg::enums::*;
use winreg::RegKey;

use std::io;
// use std::path::Path;

// HKEY_CURRENT_USER\SOFTWARE\Google\Chrome\NativeMessagingHosts\com.my_company.my_application

fn main() -> io::Result<()>  {
// fn main()   {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let (_key,disp) = hklm.create_subkey("SOFTWARE\\Google\\Chrome\\NativeMessagingHosts\\com.my_company.my_application")?;

    match disp {
        REG_CREATED_NEW_KEY => println!("A new key has been created"),
        REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
    }

    let _a:Option<i32> = Some(3);


    Ok(())
    // let pf:String = cur_ver.get_value(name: N);
    // let path = Path::new("SOFTWARE").join("path");
    // println!("{:?}",path)
}
