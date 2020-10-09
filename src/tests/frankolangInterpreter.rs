extern crate rand;
use crate::*;
use std::time::Instant;
use crate::tests::testFrankolang;

#[test]
pub fn frankolangInterpreter() {
    let code = testFrankolang();
    let mut frankolangCode = frankolang::FrankolangCode::new(&code).unwrap();
    frankolangCode.checkCode().unwrap();
    let duration = Instant::now();
    frankolangCode.executeCode().unwrap();
    println!("Executing frankolang code took {:?}", duration.elapsed());
}
