extern crate rand;
use crate::tests::testFrankolang;
use crate::*;
use std::time::Instant;

#[test]
pub fn frankolangInterpreter() {
    let code = testFrankolang();
    let mut frankolangCode =
        frankolang::FrankolangCode::new(code.to_vec()).unwrap();
    frankolangCode.checkCode().unwrap();
    let duration = Instant::now();
    frankolangCode.executeCode().unwrap();
    println!("Executing frankolang code took {:?}", duration.elapsed());
}
