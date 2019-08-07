use crossbeam::crossbeam_channel::bounded;

use std::time::Duration;
use std::thread;
use log::info;

pub fn repeat<F>(interval: Duration, fun: F) -> ()
    where F: Fn() + Send + Sync + 'static {

    let (sender, receiver) = bounded(1);

    thread::spawn(move || {
        loop {
            thread::sleep(interval);
            sender.send("tick").unwrap();
        }
    });

    thread::spawn(move || {
        loop {
            let reply = receiver.recv().unwrap();
            info!("Got reply: {}", reply);

            fun();
        };
    });
}
