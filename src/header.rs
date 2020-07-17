#![allow(non_snake_case)]

// This file is just for useful functions and such that need to be accessible globally

pub fn splitBufferAt(buffer: &[u8], pattern: u8, iterations: usize) -> Vec<&[u8]> {
    buffer.splitn(iterations, |num| *num == pattern).collect::<Vec<&[u8]>>()
}

pub fn fillBufferWith(buffer: &mut [u8], bufferFiller: &[u8], start: usize, length: usize) {
    for currentBufferByte in 0..length {
        buffer[currentBufferByte] = bufferFiller[currentBufferByte+start];
    }
}
