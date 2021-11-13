use proconio::input;
use std::collections::*;

struct Helper {
  count: usize,
  n: usize
}

impl Helper {
  fn culc(&mut self, val: usize, bit: usize) {
    if self.n < val { return }
    if bit == 7 {
      self.count += 1;
    }
    
    self.culc(val*10+3, bit | 1);
    self.culc(val*10+5, bit | 2);
    self.culc(val*10+7, bit | 4);
  }
}

fn main() {
  input! {
    n: usize,
  }
  let mut helper = Helper { n, count: 0 };
  helper.culc(0, 0);
  println!("{}", helper.count);
}