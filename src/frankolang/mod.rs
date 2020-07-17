#![allow(non_snake_case)]

use crate::ed25519_dalek;
use crate::header::*;

pub fn interpretFrankolang(code: &[u8], dryrun: bool) -> bool {
    let mut codeSegment = CodeSegment::new(code, 0);
    if !codeSegment.isSignatureValid() { return false; };
    codeSegment.moveIntructionPointerForward();

    loop {
        match codeSegment.currentInstruction() {
            0x02 => { return true }

            0x03 => {
                println!("Interpreting 0x03 instruction");
            }

            0x04 => {
                println!("Interpreting 0x04 instruction");
            }

            _ => { println!("{:#x}", codeSegment.currentInstruction()); }
        }
        codeSegment.moveIntructionPointerForward();
    }
}

struct CodeSegment<'a> {
    start: usize,
    end: usize,
    instructionPointer: usize,
    code: &'a[u8]
}

impl CodeSegment<'_> {
    fn new(code: &[u8], start: usize) -> CodeSegment {

        return CodeSegment {
            start: start,
            end: CodeSegment::findEnd(code, start),
            instructionPointer: 0,
            code: code
        }
    }

    fn moveIntructionPointerForward(&mut self) {
        self.instructionPointer += CodeSegment::bytesToNextInstruction(self.currentInstruction());
    }

    fn currentInstruction(&self) -> u8 {
        self.code[self.instructionPointer]
    }

    fn isSignatureValid(&self) -> bool {
        let publicKey = {
            let mut publicKey = [0u8; 32];
            fillBufferWith(&mut publicKey, self.code, 65, 32);

            let publicKey = ed25519_dalek::PublicKey::from_bytes(&publicKey);
            let publicKey = match publicKey {
                Ok(publicKey) => publicKey,
                Err(error) => { eprintln!("{}", error); return false },
            };
            publicKey
        };
        let signature = {
            let mut signature = [0u8; 64];
            fillBufferWith(&mut signature, self.code, 1, 64);

            let signature = ed25519_dalek::Signature::from_bytes(&signature);
            let signature = match signature {
                Ok(signature) => signature,
                Err(error) => { eprintln!("{}", error); return false },
            };
            signature
        };
        let message = {
            let messageLength = self.code.len() - 97;
            let mut message = Vec::new();
            message.resize(messageLength, 0);
            fillBufferWith(&mut message, self.code, 97, messageLength);
            message
        };

        return publicKey.verify(&message, &signature).is_ok();
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
