#![allow(unused)]

use std::fs::read_dir;

use prelude::*;

mod prelude;
mod error;
mod utils;

fn main() -> Result<()> {
    for entry in read_dir("./").unwrap().filter_map(|e| e.ok()) {
        let entry: String = W(&entry).try_into()?;
        println!("{entry}");
    }

    Ok(())
}