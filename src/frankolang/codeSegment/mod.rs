mod instructions;

use crate::cloneIntoArray;

#[derive(Clone, Debug)]
pub struct CodeSegment {
    pub end: usize,
    pub instructionPointer: usize,
    pub publicKey: ed25519_dalek::PublicKey,
    pub signature: ed25519_dalek::Signature,
    pub code: Vec<u8>,
}

impl CodeSegment {
    pub fn new(
        code: Vec<u8>,
        start: usize,
    ) -> Result<CodeSegment, ed25519_dalek::SignatureError> {
        let code = code[start..CodeSegment::findEnd(&code, start) + 1].to_vec();

        let codeSegment = CodeSegment {
            end: start + CodeSegment::findEnd(&code, start),
            instructionPointer: 97,
            publicKey: ed25519_dalek::PublicKey::from_bytes(&code[1..33])?,
            signature: ed25519_dalek::Signature::from_bytes(&code[33..97])?,
            code,
        };
        Ok(codeSegment)
    }

    pub fn nextInstruction(&mut self) {
        self.instructionPointer +=
            CodeSegment::lengthOfInstruction(self.currentInstruction());
    }

    pub fn currentInstruction(&self) -> u8 {
        self.code[self.instructionPointer]
    }

    pub fn isSignatureValid(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.publicKey
            .verify(&self.code[97..self.code.len()], &self.signature)?;
        Ok(())
    }

    pub fn lengthOfInstruction(instruction: u8) -> usize {
        match instruction {
            0x01 => 97,
            0x02 => 1,
            0x03 => 41,
            0x04 => 9,
            _ => 1,
        }
    }

    fn doesInstructionExist(&self) -> bool {
        match self.currentInstruction() {
            0x01 | 0x02 | 0x03 | 0x04 => true,
            _ => false,
        }
    }

    pub fn isSyntaxProper(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let oldInstructionPointer = self.instructionPointer;
        self.instructionPointer = 97;

        loop {
            if !self.doesInstructionExist() {
                return Err(InvalidInstructionError::fromCodeSegment(self));
            }

            if self.instructionPointer >= self.end
                || self.currentInstruction() == 0x02
            {
                break;
            }

            self.executeInstruction(true)?;
            self.nextInstruction();
        }

        self.instructionPointer = oldInstructionPointer;
        Ok(())
    }

    pub fn executeInstruction(
        &self,
        dryrun: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self.currentInstruction() {
            0x03 => {
                // Payment
                // I would rather not have to do these conversions, if there is a better and cleaner
                // way to do this that would be great, but I can't think of anything. These
                // conversions need to be done in order to use the from_le_bytes() method
                let reciever = cloneIntoArray(
                    &self.code[self.instructionPointer + 1
                        ..self.instructionPointer + 33],
                );

                let amount = {
                    let amount = cloneIntoArray(
                        &self.code[self.instructionPointer + 33
                            ..self.instructionPointer + 41],
                    );
                    u64::from_le_bytes(amount)
                };

                let mut payment = instructions::Payment::new(
                    self.publicKey.to_bytes(),
                    reciever,
                    amount,
                    dryrun,
                )?;
                payment.send()?;
            }

            0x04 => {
                // Fee
                let amount = {
                    let amount = cloneIntoArray(
                        &self.code[self.instructionPointer + 1
                            ..self.instructionPointer + 9],
                    );
                    u64::from_le_bytes(amount)
                };

                // TODO: Currently the fee is paid back to the sender. In production it should be
                // sending it to the miner of the block (but mining hasn't been implemented yet)
                let mut payment = instructions::Payment::new(
                    self.publicKey.to_bytes(),
                    self.publicKey.to_bytes(),
                    amount,
                    dryrun,
                )?;
                payment.send()?;
            }

            _ => {
                return Err(Box::new(InvalidInstructionError::fromCodeSegment(
                    self,
                )))
            }
        };
        Ok(())
    }

    fn findEnd(code: &[u8], start: usize) -> usize {
        let mut currentInstruction = start;
        loop {
            currentInstruction +=
                CodeSegment::lengthOfInstruction(code[currentInstruction]);
            if currentInstruction >= code.len() {
                break 0;
            }
            if code[currentInstruction] == 0x02 {
                break currentInstruction;
            }
        }
    }
}

#[derive(Debug)]
struct InvalidInstructionError {
    instruction: u8,
    instructionPointer: usize,
}
impl std::error::Error for InvalidInstructionError {}

impl std::fmt::Display for InvalidInstructionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "Invalid instruction (0x{:x}) at byte {}",
            self.instruction, self.instructionPointer
        )
    }
}

impl InvalidInstructionError {
    fn fromCodeSegment(
        codeSegment: &CodeSegment,
    ) -> Box<InvalidInstructionError> {
        let invalidInstructionError = InvalidInstructionError {
            instruction: codeSegment.currentInstruction(),
            instructionPointer: codeSegment.instructionPointer,
        };
        Box::new(invalidInstructionError)
    }
}
