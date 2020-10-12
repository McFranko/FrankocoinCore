#![allow(non_snake_case, dead_code)]
extern crate bincode;
extern crate dirs;
extern crate ed25519_dalek;
extern crate md5;
extern crate serde;
extern crate sha2;

mod blocks;
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

    let balanceEntriesDirPath = format!("{}/balanceEntries", dataDirPath);
    let balanceEntriesDir = Path::new(&balanceEntriesDirPath);

    let blocksDirPath = format!("{}/blocks", dataDirPath);
    let blocksDir = Path::new(&blocksDirPath);

    if !dataDir.exists() {
        fs::create_dir(dataDir)?;
    }

    if !balanceEntriesDir.exists() {
        fs::create_dir(balanceEntriesDir)?;
    }

    if !blocksDir.exists() {
        fs::create_dir(blocksDir)?;
    }

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
