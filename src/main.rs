#![allow(non_snake_case, dead_code)]
extern crate bincode;
extern crate dirs;
extern crate ed25519_dalek;
extern crate md5;
extern crate serde;
extern crate sha2;

mod frankolang;
mod server;
mod tests;

use std::fs;
use std::path::Path;

fn main() {}

fn initializeFrankocoinDirectory() -> Result<(), Box<dyn std::error::Error>> {
    let dataDirPath =
        format!("{}/frankocoin", dirs::data_dir().unwrap().to_str().unwrap());
    let dataDir = Path::new(&dataDirPath);

    if dataDir.exists() {
        return Ok(());
    }

    fs::create_dir(dataDir)?;

    let balanceEntriesDir = format!("{}/balanceEntries", dataDirPath);
    fs::create_dir(balanceEntriesDir)?;

    // Left off here
    let blocksDir = format!("{}/blocks", dataDirPath);
    fs::create_dir(blocksDir)?;

    Ok(())
}

fn cloneIntoArray<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Clone,
{
    let mut a = A::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}
