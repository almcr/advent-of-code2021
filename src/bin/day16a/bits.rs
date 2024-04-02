use std::ops::Range;

use bitvec::prelude::*;

pub struct Bits<'a> {
  view: &'a BitSlice<u8, Msb0>,
  pos: usize,
}

impl<'a> Bits<'a> {
  pub fn new(view: &'a BitSlice<u8, Msb0>) -> Self {
    Bits { view, pos: 0 }
  }

  pub fn as_u32(&self, range: Range<usize>) -> u32 {
    self.view[range].iter().fold(0, |acc, bit| (acc << 1) | (*bit) as u32)
  }

  pub fn is_empty(&self) -> bool {
    self.pos >= self.view.len()
  }

  pub fn slice_until(&mut self, at: usize) -> Bits {
    let bits = Bits {
      view: &self.view[self.pos..self.pos + at],
      pos: 0,
    };
    self.pos += at;
    bits
  }

  pub fn get(&self, i: usize) -> bool {
    self.view[i]
  }

  pub fn take_literal(&mut self) -> u32 {
    let mut sum = 0u32;
    loop {
      let stop = !self.view[self.pos];
      sum += self.as_u32(1..5);
      self.pos += 5;
      if stop {
        break;
      }
    }
    sum
  }

  pub fn take(&mut self, n: usize) -> u32 {
    self.pos += n;
    self.as_u32(self.pos - n..self.pos)
  }
}
