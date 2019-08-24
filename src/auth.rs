//
// dirtmud - Copyright (c) 2019 Ben Morrison (gbmor)
// See LICENSE file for detailed license information.
//

use std::{
    sync::mpsc,
};

pub fn user_pass(_user: &str, _pass: &[u8], _engine: mpsc::Sender<String>) -> bool {

    true
}
