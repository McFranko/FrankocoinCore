#![allow(non_snake_case)]

mod codeSegment;
mod instructions;

use codeSegment::CodeSegment;

pub fn interpretFrankolang(code: &[u8])
    -> Result<(), Box<dyn std::error::Error>>
{
    let mut startOfCodeSegment = 0;
    loop
    {
        let mut codeSegment = CodeSegment::new(code, startOfCodeSegment)?;

        codeSegment.isSignatureValid()?;
        codeSegment.isSyntaxProper()?;
        
        if codeSegment.end+1 >= code.len() - 1 || code[codeSegment.end+1] == 0x0f
        {
            break;
        }

        startOfCodeSegment = codeSegment.end + 1;
    }

    Ok(())
}

