#![allow(dead_code)]
extern crate dirs;

#[macro_use]
extern crate lazy_static;


mod server;
mod threadpool;

#[cfg(test)]
mod tests;

lazy_static! {
    static ref FRANKOCOIN_DIRECTORY: String = {
        format!("{}/frankocoin", dirs::data_dir().unwrap().to_str().unwrap())
    };
}

fn main() {}


