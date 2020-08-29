#![allow(non_snake_case)]

mod codeSegment;
mod instructions;

use codeSegment::CodeSegment;

pub fn interpretFrankolang(code: &[u8]) -> bool
{
    let mut startOfCodeSegment = 0;
    loop
    {
        let mut codeSegment = match CodeSegment::new(code, startOfCodeSegment)
        {
            Ok(codeSegment) => codeSegment,
            Err(_) => {
                return false;
            }
        };

        if !codeSegment.isSyntaxProper() || !codeSegment.isSignatureValid()
        {
            return false;
        }
        
        if codeSegment.end >= code.len() - 1
        {
            break;
        }

        startOfCodeSegment = codeSegment.end + 1;
    }

    true
}

