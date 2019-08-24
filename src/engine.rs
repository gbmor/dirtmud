//
// dirtmud - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use std::{
    sync::mpsc,
    thread,
};

pub fn spawn_worker(rx: mpsc::Receiver<String>) {
    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(comm) => println!("{}", comm),
                Err(err) => eprintln!("{}", err),
            }
        }
    });
}
