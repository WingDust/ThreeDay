
#[macro_use(println_stderr)]
extern crate webextension_protocol as protocol;
use std::io::Write;
use std::process;

use std::fs::File;

fn main() {
    loop {
        let message = match protocol::read_stdin() {
            Ok(m) => m,
            Err(_) => process::exit(1),
        };
        if message != "" {
            let f =  File::open("backup.json");
            match File::create("backup.json"){
                Ok(_) =>{ },

                Err(_) => {
                        // panic!("")
                        println_stderr!("create failed");
                }
            }
        }

        println_stderr!("received {}", message);
        protocol::write_stdout(message);
    }
}