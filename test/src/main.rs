#![feature(asm)] 
#![feature(plugin)]
#![plugin(bytestool)]
#[macro_use(releasetag)]
extern crate releasetag;

use std::time::Duration;
use std::thread;
use std::io::stdout;
use std::io::Write;

fn main() {
    // test with tag larger than 32 bytes
    releasetag!(b"BUILD_TAG=MAIN_2016-wk16-05-AAAA-BBBB-CCCC-DDDD-EEEE-FFFF-GGGG-HHHH-IIII-JJJJ-KKKK");
    releasetag!(b"BUILD_HOST=host1");

    println!("Waiting until being aborted");
    loop {
      thread::sleep(Duration::from_millis(200));
      print!(".");
      stdout().flush().ok();
    }
}
