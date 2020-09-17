#![allow(non_snake_case)]
extern crate md5;
extern crate ed25519_dalek;
extern crate serde;
extern crate bincode;
extern crate dirs;

mod frankolang;
mod server;
mod tests;

use std::path::Path;
use std::fs;

fn main() {

}

fn initializeFrankocoinDirectory()
    -> Result<(), Box<dyn std::error::Error>>
{
    let dataDirPath = format!(
        "{}/.frankocoin",
        dirs::home_dir().unwrap().to_str().unwrap()
    );
    let dataDir = Path::new(&dataDirPath);

    if dataDir.exists() {
        return Ok(())
    }

    fs::create_dir(dataDir)?;

    let balanceEntriesDir = format!(
        "{}/balanceEntries",
        dataDirPath
    );
    fs::create_dir(balanceEntriesDir)?;

    // Left off here
    let blocksDir = format!(
        "{}/blocks",
        dataDirPath
    );
    fs::create_dir(blocksDir);

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
