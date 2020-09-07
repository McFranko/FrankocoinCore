use crate::frankolang::instructions;

#[derive(Debug)]
pub struct CodeSegment<'a>
{
    pub end: usize,
    instructionPointer: usize,
    pub publicKey: ed25519_dalek::PublicKey,
    pub signature: ed25519_dalek::Signature,
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
            publicKey: ed25519_dalek::PublicKey::from_bytes(&code[1..33])?,
            signature: ed25519_dalek::Signature::from_bytes(&code[33..97])?,
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

    fn currentInstruction(&self) 
        -> u8
    {
        self.code[self.instructionPointer]
    }

    pub fn isSignatureValid(&self)
        -> Result<(), Box<dyn std::error::Error>> 
    {
        self.publicKey.verify(
            &self.code[97..self.code.len()],
            &self.signature
        )?;
        Ok(())
    }

    fn lengthOfInstruction(instruction: u8)
        -> usize
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

    fn doesInstructionExist(&self)
        -> bool
    {
        match self.currentInstruction()
        {
            0x01 | 0x02 | 0x03 | 0x04 => true,
            _ => false
        }
    }

    pub fn isSyntaxProper(&mut self)
        -> Result<(), Box<dyn std::error::Error>>
    {
        let oldInstructionPointer = self.instructionPointer;
        self.instructionPointer = 0;

        loop
        {
            if !self.doesInstructionExist()
            {
                return Err(
                    Box::new(
                        std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!(
                                "Improper instruction at {}", self.instructionPointer
                            )
                        )
                    )
                );
            }
            if self.instructionPointer >= self.end
            {
                break;
            }
            self.nextInstruction();
        }

        self.instructionPointer = oldInstructionPointer;
        Ok(())
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
                // let amount = &self.code[self.instructionPointer+33+self];
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

    fn findEnd(code: &[u8], start: usize)
        -> usize
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
