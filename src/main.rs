//
// dirtmud - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use std::{
    sync::mpsc,
};

use chrono::prelude::*;

mod clients;
mod engine;

fn main() {
    let thetime = Utc::now();
    println!();
    println!("dirtmud 0.1-dev");
    println!("{}", thetime.to_rfc2822());

    let (tx, rx) = mpsc::channel::<String>();
    engine::spawn_worker(rx);
    clients::spawn_worker("0.0.0.0:56543", tx).unwrap();
}
