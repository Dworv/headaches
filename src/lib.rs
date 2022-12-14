use std::io::{stdin, stdout, Read, Write};

pub mod compat;
pub mod interpret;

use compat::from_char_8859;
pub use interpret::{execute, parse, run, run_from_state};

pub use compat::to_char_8859;

/// A `Vec` of `u8`s representing a the memory
/// of a Brainfuck process.
pub type Memory = Vec<u8>;
/// The pointer that indicates the selected cell
/// of a Brainfuck process.
pub type Pointer = usize;

/// The state of a Brainfuck process.
#[derive(Debug)]
pub struct State {
    /// The process's memory.
    pub mem: Memory,
    /// The location of the pointer.
    pub pointer: Pointer,
    /// Whether the process has outputted.
    ///
    /// Used for repl
    pub outted: bool,
}

impl State {
    /// Creates a new Brainfuck state.
    pub fn new() -> Self {
        Self {
            mem: vec![0],
            pointer: 0,
            outted: false,
        }
    }
    #[allow(unused_must_use)]
    pub fn run(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Increment => {
                self.mem[self.pointer] = self.mem[self.pointer].overflowing_add(1).0;
            }
            Instruction::Decrement => {
                self.mem[self.pointer] = self.mem[self.pointer].overflowing_sub(1).0;
            }
            Instruction::Forward => {
                if self.pointer + 1 == self.mem.len() {
                    self.mem.push(0)
                }
                self.pointer += 1;
            }
            Instruction::Backward => {
                if self.pointer != 0 {
                    self.pointer -= 1;
                }
            }
            Instruction::Loop(inners) => loop {
                for inner in inners {
                    self.run(inner)
                }
                if self.mem[self.pointer] == 0 {
                    break;
                }
            },
            Instruction::LoopEnd => {}
            Instruction::Out => {
                self.outted = true;
                print!("{}", to_char_8859(self.mem[self.pointer]));
                stdout().flush();
            }
            Instruction::In => {
                if let Some(Ok(c)) = stdin().bytes().next() {
                    let b = from_char_8859(c as char);
                    self.mem[self.pointer] = b;
                    self.outted = true;
                }
            }
        }
    }
}

/// A Brainfuck instruction.
#[derive(Debug)]
pub enum Instruction {
    /// Represents the `+` instruction.
    ///
    /// Used to increment the currect cell.
    Increment,
    /// Represents the `-` instruction.
    ///
    /// Used to decrement the currect cell.
    Decrement,
    /// Represents the `>` instruction.
    ///
    /// Used to move to the next cell.
    Forward,
    /// Represents the `<` instruction.
    ///
    /// Used to increment the previous cell.
    Backward,
    /// Represents a loop (inside `[]`).
    ///
    /// Repeats everything within it for as long as the pointer
    /// starts on a non-null cell.
    Loop(Vec<Instruction>),
    /// Represents the end of a Brainfuck loop.
    ///
    /// Only used for parsing, not present in a parsed list
    /// of instructions.
    LoopEnd,
    /// Represents the `.` instruction.
    ///
    /// Used to increment the previous cell.
    Out,
    /// Represents the `,` instruction.
    ///
    /// Opens stdin and takes the first character of the inputted
    /// string.
    In,
}

#[derive(Debug)]
pub struct TryFromCharError;

impl std::fmt::Display for TryFromCharError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not convert from a `char`.")
    }
}

impl std::error::Error for TryFromCharError {}

impl TryFrom<char> for Instruction {
    type Error = TryFromCharError;
    /// Parses a character into a Brainfuck instruction.
    ///
    /// If the input is not an instruction,
    fn try_from(c: char) -> Result<Self, TryFromCharError> {
        use Instruction::*;
        Ok(match c {
            '+' => Increment,
            '-' => Decrement,
            '>' => Forward,
            '<' => Backward,
            '[' => Loop(vec![]),
            ']' => LoopEnd,
            '.' => Out,
            ',' => In,
            _ => return Err(TryFromCharError),
        })
    }
}
