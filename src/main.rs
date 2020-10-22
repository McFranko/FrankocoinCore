#![allow(dead_code)]
extern crate bincode;
extern crate dirs;
extern crate ed25519_dalek;
extern crate md5;
extern crate serde;
extern crate sha2;

mod frankolang;
mod server;
mod tests;
mod threadpool;

use std::fs;
use std::path::Path;

fn main() {}

fn init_frankocoin_directory() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir_path =
        format!("{}/frankocoin", dirs::data_dir().unwrap().to_str().unwrap());
    let data_dir = Path::new(&data_dir_path);

    let balance_entries_dir_path = format!("{}/balanceEntries", data_dir_path);
    let balance_entries_dir = Path::new(&balance_entries_dir_path);

    let blocks_dir_path = format!("{}/blocks", data_dir_path);
    let blocks_dir = Path::new(&blocks_dir_path);

    if !data_dir.exists() {
        fs::create_dir(data_dir)?;
    }

    if !balance_entries_dir.exists() {
        fs::create_dir(balance_entries_dir)?;
    }

    if !blocks_dir.exists() {
        fs::create_dir(blocks_dir)?;
    }

    Ok(())
}

fn clone_into_array<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Clone,
{
    let mut a = A::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}
