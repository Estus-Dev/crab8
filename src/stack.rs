use crate::prelude::*;
use std::{fmt, fmt::Debug, fmt::Display, fmt::Formatter};

/// The CHIP-8 stack should support a minimum of 12 frames, but implementations are encouraged to
/// allow for larger sizes.
const MAX_STACK_DEPTH: usize = 16;

#[derive(Clone, PartialEq, Eq)]
/// The stack holds all previous PC values while executing one or more subroutines.
/// The CHIP-8 requires at least 12 frames, but modern interpreters are encouraged to go higher.
pub struct Stack {
    stack: [Address; MAX_STACK_DEPTH],
    length: usize,
}

impl Stack {
    /// Create a new empty Stack.
    pub fn empty() -> Self {
        Self {
            stack: [Default::default(); MAX_STACK_DEPTH],
            length: 0,
        }
    }

    /// Push a new address to the stack.
    /// UNDEFINED BEHAVIOR: It's not clear what should happen when pushing too many things onto the
    /// stack.
    pub fn push(&mut self, address: Address) -> Result<(), ()> {
        if self.length < MAX_STACK_DEPTH {
            self.stack[self.length] = address;
            self.length += 1;

            Ok(())
        } else {
            Err(())
        }
    }

    /// Pop the latest address off the stack.
    /// UNDEFINED BEHAVIOR: It's unspecified what happens when a value is popped off an empty stack.
    pub fn pop(&mut self) -> Result<Address, ()> {
        if self.length > 0 {
            self.length -= 1;

            Ok(self.stack[self.length])
        } else {
            Err(())
        }
    }

    /// Get the current length of the stack.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Check to see if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.len() < 1
    }
}

impl Debug for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let debug_string = self
            .stack
            .iter()
            .take(self.length)
            .map(|addr| format!("{addr}"))
            .rev()
            .fold("".to_owned(), |debug_string, addr| {
                debug_string + " " + &addr
            })
            .trim()
            .to_owned();

        write!(f, "{debug_string}")
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::empty()
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl IntoIterator for Stack {
    type Item = Address;
    type IntoIter = StackIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            stack: self,
            index: 0,
        }
    }
}

pub struct StackIterator {
    stack: Stack,
    index: usize,
}

impl Iterator for StackIterator {
    type Item = Address;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.stack.length {
            let current_index = self.index;
            self.index += 1;
            Some(self.stack.stack[current_index])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stack() -> Result<(), ()> {
        let mut stack = Stack::empty();

        assert_eq!(stack.len(), 0);
        assert!(stack.is_empty());

        stack.push(0x123.try_into()?)?;

        assert_eq!(stack.len(), 1);
        assert!(!stack.is_empty());

        stack.push(0x234.try_into()?)?;

        assert_eq!(stack.len(), 2);
        assert!(!stack.is_empty());

        stack.push(0x345.try_into()?)?;

        assert_eq!(stack.len(), 3);
        assert!(!stack.is_empty());

        assert_eq!(stack.pop()?, 0x345.try_into()?);
        assert_eq!(stack.len(), 2);
        assert!(!stack.is_empty());

        stack.push(0x456.try_into()?)?;

        assert_eq!(stack.len(), 3);
        assert!(!stack.is_empty());

        assert_eq!(stack.pop()?, 0x456.try_into()?);
        assert_eq!(stack.len(), 2);
        assert!(!stack.is_empty());

        assert_eq!(stack.pop()?, 0x234.try_into()?);
        assert_eq!(stack.len(), 1);
        assert!(!stack.is_empty());

        assert_eq!(stack.pop()?, 0x123.try_into()?);
        assert_eq!(stack.len(), 0);
        assert!(stack.is_empty());

        assert!(stack.pop().is_err());

        Ok(())
    }

    #[test]
    fn test_stack_overflow() -> Result<(), ()> {
        let mut stack = Stack::empty();

        for i in 0..MAX_STACK_DEPTH {
            stack.push((i as u16).try_into()?)?;
        }

        assert!(stack.push(0x000.try_into()?).is_err());

        Ok(())
    }
}
