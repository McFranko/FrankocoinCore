use crate::frankolang::instructions;

pub struct CodeSegment<'a>
{
    pub end: usize,
    instructionPointer: usize,
    publicKey: ed25519_dalek::PublicKey,
    signature: ed25519_dalek::Signature,
    code: &'a[u8]
}

impl CodeSegment<'_>
{
    pub fn new(code: &[u8], start: usize)
        -> Result<CodeSegment, ed25519_dalek::SignatureError>
    {
        let code = &code[start..CodeSegment::findEnd(code, start)+1];

        let codeSegment = CodeSegment
        {
            end: start + CodeSegment::findEnd(code, start),
            instructionPointer: 0,
            publicKey: ed25519_dalek::PublicKey::from_bytes(&code[65..97])?,
            signature: ed25519_dalek::Signature::from_bytes(&code[1..65])?,
            code: code
        };
        Ok(codeSegment)
    }

    fn nextInstruction(&mut self)
    {
        self.instructionPointer += CodeSegment::lengthOfInstruction(
            self.currentInstruction()
        );
    }

    fn currentInstruction(&self) -> u8
    {
        self.code[self.instructionPointer]
    }

    pub fn isSignatureValid(&self) -> bool
    {
        instructions::BalanceEntry::fromKey(&self.publicKey.to_bytes()).unwrap();
        self.publicKey.verify(
            &self.code[97..self.code.len()], &self.signature
        ).is_ok()
    }

    fn lengthOfInstruction(instruction: u8) -> usize
    {
        match instruction
        {
            0x01 => 97,
            0x02 => 1,
            0x03 => 41,
            0x04 => 9,
            _ => 1
        }
    }

    fn doesInstructionExist(&self) -> bool
    {
        match self.currentInstruction()
        {
            0x01 | 0x02 | 0x03 | 0x04 => true,
            _ => false
        }
    }

    pub fn isSyntaxProper(&mut self) -> bool
    {
        let oldInstructionPointer = self.instructionPointer;
        self.instructionPointer = 0;

        loop
        {
            if !self.doesInstructionExist()
            {
                return false;
            }
            if self.instructionPointer >= self.end
            {
                break;
            }
            self.nextInstruction();
        }

        self.instructionPointer = oldInstructionPointer;
        true
    }

    fn executeInstruction(&self)
    {
        match self.currentInstruction()
        {
            0x03 =>
            {
                // payment
                // get parameters
                let reciever = &self.code[
                    self.instructionPointer+1..self.instructionPointer+33
                ];
                let sender = self.publicKey.to_bytes();
                let amount = &self.code[self.instructionPointer+33+self];
                // add parameters to Payment struct and get on with it
            }
            0x04 =>
            {
                //miner fee
            }
            _ =>
            {
                //error handling
            }
        }
    }

    fn findEnd(code: &[u8], start: usize) -> usize
    {
        let mut currentInstruction = start;
        loop
        {
            currentInstruction += CodeSegment::lengthOfInstruction(
                code[currentInstruction]
            );

            if currentInstruction >= code.len()
            {
                break 0;
            }
            if code[currentInstruction] == 0x02
            {
                break currentInstruction;
            }
        }
    }
}
