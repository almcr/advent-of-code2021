#![feature(iter_array_chunks)]

mod bits;
pub use bits::Bits;
use bitvec::{order::Msb0, view::BitView};

fn parse(bites: &mut Bits) -> u32 {
  let mut ver = bites.take(3);
  let type_id = bites.take(3);
  if type_id == 4 {
    let _ = bites.take_literal();
    return ver;
  }

  if bites.take(1) == 0 {
    let num_bites = bites.take(15);
    let sub_packets = &mut bites.slice_until(num_bites as usize);
    while !sub_packets.is_empty() {
      ver += parse(sub_packets)
    }
  } else {
    let num_packets = bites.take(11);
    (0..num_packets).for_each(|_| ver += parse(bites));
  }
  ver
}

fn main() {
  let mut bites = include_str!("input")
    .trim()
    .chars()
    .array_chunks()
    .map(|[a, b]| (a.to_digit(16).unwrap() << 4) as u8 | b.to_digit(16).unwrap() as u8)
    .collect::<Vec<_>>();

  let bites = bites.view_bits_mut::<Msb0>();

  let sum = parse(&mut Bits::new(bites));

  println!("{}", sum);
}
