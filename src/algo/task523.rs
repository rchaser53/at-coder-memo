use std::collections::*;
struct Helper {
  memo: Vec<usize>,
}

impl Helper {
  fn culc(&mut self, a:usize) {
    let mut i = 1;
    let mut set = HashSet::new();
    while i * i <= a {
      if a % i == 0 {
        set.insert(i);
        set.insert(a/i);
      }
      i += 1;
    }

    if set.len() == 6 {
      self.memo.push(a);
    }
  }
}

fn main() {
  let mut helper = Helper { memo:vec![] };
  
  let mut i = 1;
  while helper.memo.len() < 5 {
    helper.culc(i);
    i += 1;
  }

  let mut result = helper.memo;
  result.sort();
  for i in 0..5 {
    println!("{}", result[i]);
  }
}