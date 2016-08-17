#![feature(asm)] 
#[macro_use(releasetag)]
extern crate releasetag;

use std::time::Duration;
use std::thread;
use std::io::stdout;
use std::io::Write;
fn main() {
    releasetag!("BUILD_TAG=MAIN_2016-wk16-05");
    releasetag!("BUILD_HOST=host1");

    println!("Waiting until being aborted");
    loop {
      thread::sleep(Duration::from_millis(200));
      print!(".");
      stdout().flush().ok();      
    }
}