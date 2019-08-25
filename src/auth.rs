//
// dirtmud - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use bcrypt;
use log::{
    warn,
};

use crate::json_local;

pub fn user_pass(user: &str, pass: &[u8]) -> bool {
    let userdata = json_local::pull_user_data();
    let stored_hash = json::stringify(userdata[
        &format!("{}.hash", user)
    ].clone()).to_string();

    match bcrypt::verify(&pass, &stored_hash) {
        Ok(resp) => return resp,
        Err(_) => {
            warn!("Invalid auth for user: {}", user);
            return false
        }
    }
}
