use std::collections::{HashMap, BTreeMap};

use serde::{Serialize, Deserialize};

/// one word = 4 bytes
/// a 32 bit word must be located
/// and accessed using a word aligned
/// address. This means that the address
/// must be divisible by 4 or the last
/// two bits must be 0.
struct Word {
    bytes: [u8; 4],
}

/// half word = 2 bytes
/// a 16 bit half word must be located
/// and accessed using a half word aligned 
/// address. This means that the address
/// the last lower order bit must be 0.
struct HalfWord {
    bytes: [u8; 2],
}

/// data assembler directive
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum DataDirective {
    // stores n values in successive bytes of memory
    Byte(u8),
    // stores n 16 bit values in successive half words of memory
    HalfWord(u16),
    // stores n 32 bit values in successive words of memory
    Word(u32),
    // ascii string without null terminator
    Ascii(String),
    // ascii string with null terminator
    AsciiZero(String),
    // space for n bytes
    Space(u32),
    // aligns the next data on a word boundary
    Align(u32),
}

/// returned by parser
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct DataMap {
    name: String,
    data: DataDirective,
}

impl DataMap {

    pub fn new(name: String, data: DataDirective) -> DataMap {
        DataMap {
            name,
            data,
        }
    }
        
}

/// each memory write is tagged without 
/// taking up extra space in the memory itself,
/// this tag is useful for debugging as the memory
/// can be shown as strings, integers, floats, etc.
/// and not just bytes.
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum MemTag {
    String,
}

/// represents the memory of the system
/// and stores the data section, the text
/// resides in the program counter
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Memory {
    data: Vec<u8>,
    tags: BTreeMap<usize, Option<MemTag>>,
}

impl Memory {

    pub fn new() -> Memory {
        Memory {
            data: Vec::new(),
            tags: BTreeMap::new(),
        }
    }

    pub fn last(&self) -> usize {
        self.data.len()
    }

    pub fn tag(&mut self, addr: usize, tag: MemTag) {
        self.tags.insert(addr, Some(tag));
    }

    pub fn write(&mut self, addr: usize, data: &[u8]) {
        let mut i = 0;
        for byte in data {
            self.data[addr + i] = *byte;
            i += 1;
        }
    }

    pub fn read(&self, addr: usize) -> (u8, Option<MemTag>) {
        (self.data[addr], self.tags.get(&addr).unwrap().clone())
    }

}

