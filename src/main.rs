//
// dirtmud - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use chrono::prelude::*;

fn main() {
    let thetime = Utc::now();
    println!();
    println!("dirtmud 0.1-dev");
    println!("{}", thetime.to_rfc2822());
}
