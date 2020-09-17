#![allow(non_snake_case)]

mod codeSegment;

use codeSegment::CodeSegment;

pub struct FrankolangCode<'a> {
    code: &'a [u8],
    codeSegments: Vec<CodeSegment<'a>>
}

impl FrankolangCode<'_> {
    pub fn new(code: &[u8])
        -> Result<FrankolangCode, Box<dyn std::error::Error>>
    {
        let mut codeSegments: Vec<CodeSegment> = Vec::new();
        let mut startOfCodeSegment = 0;
        loop {
            let codeSegment = CodeSegment::new(code, startOfCodeSegment)?;

            codeSegments.push(codeSegment.clone());

            if codeSegment.end+1 >= code.len()-1 || code[codeSegment.end+1] == 0x0f {
                break;
            }
            
            startOfCodeSegment = codeSegment.end + 1;
        };

        let frankolangCode = FrankolangCode {
            code,
            codeSegments
        };

        Ok(frankolangCode)
    }

    pub fn checkCode(&mut self)
        -> Result<(), Box<dyn std::error::Error>>
    {
        for codeSegment in self.codeSegments.iter_mut() {
            codeSegment.isSignatureValid()?;
            codeSegment.isSyntaxProper()?;
        }
        Ok(())
    }

    pub fn executeCode(&mut self)
        -> Result<(), Box<dyn std::error::Error>>
    {
        for codeSegment in self.codeSegments.iter_mut() {
            loop {
                codeSegment.executeInstruction()?;
                codeSegment.nextInstruction();
                if codeSegment.currentInstruction() == 0x02 {
                    break;
                }
            }
        }
        Ok(())
    }
}
