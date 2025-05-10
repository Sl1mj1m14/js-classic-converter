use std::fs::{self, OpenOptions};
use std::io::Write;
use std::process::exit;

use serde::Deserialize;
use thiserror::Error;
use rusqlite::Error;

use mc_classic;
use mc_classic_js;

mod convert;

pub fn main () {
    println!("Hello world!");
}