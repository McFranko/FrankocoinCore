extern crate rand;
use crate::tests::test_frankolang;
use crate::*;
use std::time::Instant;

#[test]
pub fn frankolang_interpreter() {
    let code = test_frankolang();
    let mut frankolang_code =
        frankolang::FrankolangCode::new(code.to_vec()).unwrap();
    frankolang_code.check_code().unwrap();
    let duration = Instant::now();
    frankolang_code.execute_code().unwrap();
    println!("Executing frankolang code took {:?}", duration.elapsed());
}
