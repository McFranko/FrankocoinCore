pub mod code_segment;

use code_segment::CodeSegment;

pub struct FrankolangCode {
    pub code: Vec<u8>,
    pub code_segments: Vec<CodeSegment>,
}

impl FrankolangCode {
    pub fn new(
        code: Vec<u8>,
    ) -> Result<FrankolangCode, Box<dyn std::error::Error>> {
        let mut code_segments: Vec<CodeSegment> = Vec::new();
        let mut start_of_code_segment = 0;
        loop {
            let code_segment =
                CodeSegment::new(code.clone(), start_of_code_segment)?;

            code_segments.push(code_segment.clone());

            if code_segment.end + 1 >= code.len() - 1
                || code[code_segment.end + 1] == 0x0f
            {
                break;
            }

            start_of_code_segment = code_segment.end + 1;
        }

        let frankolang_code = FrankolangCode {
            code,
            code_segments,
        };

        Ok(frankolang_code)
    }

    pub fn check_code(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for code_segment in self.code_segments.iter_mut() {
            code_segment.is_signature_valid()?;
            code_segment.is_syntax_proper()?;
        }
        Ok(())
    }

    pub fn execute_code(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for code_segment in self.code_segments.iter_mut() {
            loop {
                code_segment.execute_instruction(false)?;
                code_segment.next_instruction();
                if code_segment.current_instruction() == 0x02 {
                    break;
                }
            }
        }
        Ok(())
    }
}
