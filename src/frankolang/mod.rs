#![allow(non_snake_case)]
use crate::ed25519_dalek;

//pub mod payments;

pub fn interpretFrankolang(code: &[u8], dryrun: bool) -> bool {
    let mut startOfCodeSegment = 0;

    loop {
        let mut codeSegment = match CodeSegment::new(code, startOfCodeSegment) {
            Ok(codeSegment) => codeSegment,
            Err(_) => break false
        };

        if !codeSegment.isSignatureValid() { break false; };
        codeSegment.moveIntructionPointerForward();

        loop {
            match codeSegment.currentInstruction() {
                0x02 => { break }

                0x03 => {
                }

                0x04 => {
                }

                _ => { println!("{:#x}", codeSegment.currentInstruction()); }
            }
            codeSegment.moveIntructionPointerForward();
        }

        if codeSegment.end >= code.len()-1 {
            break true;
        } else {
            startOfCodeSegment = codeSegment.end;
        }
    }
}

struct CodeSegment<'a> {
    end: usize,
    instructionPointer: usize,
    publicKey: ed25519_dalek::PublicKey,
    signature: ed25519_dalek::Signature,
    code: &'a[u8]
}

impl CodeSegment<'_> {
    fn new(code: &[u8], start: usize) -> Result<CodeSegment, ed25519_dalek::SignatureError> {
        let code = &code[start..CodeSegment::findEnd(code, start)+1];
        let publicKey = ed25519_dalek::PublicKey::from_bytes(&code[65..97])?;
        let signature = ed25519_dalek::Signature::from_bytes(&code[1..65])?;

        let codeSegment = CodeSegment {
            end: start + CodeSegment::findEnd(code, start),
            instructionPointer: 0,
            publicKey: publicKey,
            signature: signature,
            code: code
        };
        Ok(codeSegment)
    }

    fn moveIntructionPointerForward(&mut self) {
        self.instructionPointer += CodeSegment::bytesToNextInstruction(self.currentInstruction());
    }

    fn currentInstruction(&self) -> u8 {
        self.code[self.instructionPointer]
    }

    fn isSignatureValid(&self) -> bool {
        self.publicKey.verify(&self.code[97..self.code.len()], &self.signature).is_ok()
    }

    fn bytesToNextInstruction(instruction: u8) -> usize {
        match instruction {
            0x01 => 97,
            0x02 => 1,
            0x03 => 73,
            0x04 => 9,
            _ => 1
        }
    }

    fn findEnd(code: &[u8], start: usize) -> usize {
        let mut currentInstruction = start;
        loop {
            currentInstruction += CodeSegment::bytesToNextInstruction(code[currentInstruction]);
            if currentInstruction > code.len() - 1 { break 0; }
            if code[currentInstruction] == 0x02 { break currentInstruction; };
        }
    }
}
