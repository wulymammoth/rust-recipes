use std::{thread, time};

pub fn main() {
    let pb = indicatif::ProgressBar::new(10);
    for i in 0..10 {
        do_expensive_work();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}

fn do_expensive_work() {
    thread::sleep(time::Duration::from_millis(100));
}
