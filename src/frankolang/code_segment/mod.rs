mod instructions;

use crate::clone_into_array;

#[derive(Clone, Debug)]
pub struct CodeSegment {
    pub end: usize,
    pub instruction_pointer: usize,
    pub public_key: ed25519_dalek::PublicKey,
    pub signature: ed25519_dalek::Signature,
    pub code: Vec<u8>,
}

impl CodeSegment {
    pub fn new(
        code: Vec<u8>,
        start: usize,
    ) -> Result<CodeSegment, ed25519_dalek::SignatureError> {
        let code =
            code[start..CodeSegment::find_end(&code, start) + 1].to_vec();

        let code_segment = CodeSegment {
            end: start + CodeSegment::find_end(&code, start),
            instruction_pointer: 97,
            public_key: ed25519_dalek::PublicKey::from_bytes(&code[1..33])?,
            signature: ed25519_dalek::Signature::from_bytes(&code[33..97])?,
            code,
        };
        Ok(code_segment)
    }

    pub fn next_instruction(&mut self) {
        self.instruction_pointer +=
            CodeSegment::length_of_instruction(self.current_instruction());
    }

    pub fn current_instruction(&self) -> u8 {
        self.code[self.instruction_pointer]
    }

    pub fn is_signature_valid(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.public_key
            .verify(&self.code[97..self.code.len()], &self.signature)?;
        Ok(())
    }

    pub fn length_of_instruction(instruction: u8) -> usize {
        match instruction {
            0x01 => 97,
            0x02 => 1,
            0x03 => 41,
            0x04 => 9,
            _ => 1,
        }
    }

    fn does_instruction_exist(&self) -> bool {
        match self.current_instruction() {
            0x01 | 0x02 | 0x03 | 0x04 => true,
            _ => false,
        }
    }

    pub fn is_syntax_proper(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let old_instruction_pointer = self.instruction_pointer;
        self.instruction_pointer = 97;

        loop {
            if !self.does_instruction_exist() {
                return Err(InvalidInstructionError::from_code_segment(self));
            }

            if self.instruction_pointer >= self.end
                || self.current_instruction() == 0x02
            {
                break;
            }

            self.execute_instruction(true)?;
            self.next_instruction();
        }

        self.instruction_pointer = old_instruction_pointer;
        Ok(())
    }

    pub fn execute_instruction(
        &self,
        dryrun: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self.current_instruction() {
            0x03 => {
                // Payment
                // I would rather not have to do these conversions, if there is a better and cleaner
                // way to do this that would be great, but I can't think of anything. These
                // conversions need to be done in order to use the from_le_bytes() method
                let reciever = clone_into_array(
                    &self.code[self.instruction_pointer + 1
                        ..self.instruction_pointer + 33],
                );

                let amount = {
                    let amount = clone_into_array(
                        &self.code[self.instruction_pointer + 33
                            ..self.instruction_pointer + 41],
                    );
                    u64::from_le_bytes(amount)
                };

                let mut payment = instructions::Payment::new(
                    self.public_key.to_bytes(),
                    reciever,
                    amount,
                    dryrun,
                )?;
                payment.send()?;
            }

            0x04 => {
                // Fee
                let amount = {
                    let amount = clone_into_array(
                        &self.code[self.instruction_pointer + 1
                            ..self.instruction_pointer + 9],
                    );
                    u64::from_le_bytes(amount)
                };

                // TODO: Currently the fee is paid back to the sender. In production it should be
                // sending it to the miner of the block (but mining hasn't been implemented yet)
                let mut payment = instructions::Payment::new(
                    self.public_key.to_bytes(),
                    self.public_key.to_bytes(),
                    amount,
                    dryrun,
                )?;
                payment.send()?;
            }

            _ => {
                return Err(Box::new(
                    InvalidInstructionError::from_code_segment(self),
                ))
            }
        };
        Ok(())
    }

    fn find_end(code: &[u8], start: usize) -> usize {
        let mut current_instruction = start;
        loop {
            current_instruction +=
                CodeSegment::length_of_instruction(code[current_instruction]);
            if current_instruction >= code.len() {
                break 0;
            }
            if code[current_instruction] == 0x02 {
                break current_instruction;
            }
        }
    }
}

#[derive(Debug)]
struct InvalidInstructionError {
    instruction: u8,
    instruction_pointer: usize,
}
impl std::error::Error for InvalidInstructionError {}

impl std::fmt::Display for InvalidInstructionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "Invalid instruction (0x{:x}) at byte {}",
            self.instruction, self.instruction_pointer
        )
    }
}

impl InvalidInstructionError {
    fn from_code_segment(
        code_segment: &CodeSegment,
    ) -> Box<InvalidInstructionError> {
        let invalid_instruction_error = InvalidInstructionError {
            instruction: code_segment.current_instruction(),
            instruction_pointer: code_segment.instruction_pointer,
        };
        Box::new(invalid_instruction_error)
    }
}
