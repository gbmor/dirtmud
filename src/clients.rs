//
// dirtmud - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use std::{
    io,
    net::{
        TcpListener,
    },
    sync::mpsc,
    thread,
};


pub fn spawn_worker(ip: &str, tx: mpsc::Sender<String>) -> Result<(), io::Error> {
    match TcpListener::bind(ip) {
        Ok(lstnr) => {
            for conn in lstnr.incoming() {
                match conn {
                    Ok(strm) => {
                        strm.set_nonblocking(true)?;
                        let _txc = tx.clone();
                        thread::spawn(move || {
                            println!("Placeholder");
                        });
                    }
                    Err(err) => eprintln!("{}", err),
                }
            }
        }
        Err(err) => return Err(err),
    }
    Ok(())
}
