#![allow(non_snake_case)]
use ed25519_dalek;
use rand;

use crate::frankolang;

pub fn runTests()
{
    let start = std::time::SystemTime::now();
    let result = frankolangInterpreterTest();
    let duration = start.elapsed().expect("clock may have gone backwards");
    println!("Interpreter test returned {} after {:?}", result, duration);
}

fn frankolangInterpreterTest() -> bool
{
    let mut message: [0u8:]

    true
}
