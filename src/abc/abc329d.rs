/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

impl PartialEq for Wrapper {
  fn eq(&self, other: &Wrapper) -> bool {
    self.0 == other.0
  }
}
impl PartialOrd for Wrapper {
  fn partial_cmp(&self, other: &Wrapper) -> Option<Ordering> {
    let v = self.0.cmp(&other.0);
    Some(if v == Ordering::Equal {
      self.1.cmp(&other.1)
    } else {
      v
    })
  }
}

impl Ord for Wrapper {
  fn cmp(&self, other: &Wrapper) -> Ordering {
    let v = self.0.cmp(&other.0);
    if v == Ordering::Equal {
      self.1.cmp(&other.1)
    } else if v == Ordering::Less {
      Ordering::Greater
    } else {
      Ordering::Less
    }
  }
}

#[derive(Clone, Copy, Eq)] 
struct Wrapper(usize,usize);

fn main() {
  input! {
    n:usize,
    m:usize,
    a:[Usize1;m]
  }

  let mut btreeset = BTreeSet::new();
  let mut memo = vec![0;n];
  for i in a {
    if memo[i] == 0 {
      memo[i] += 1;
      btreeset.insert(Wrapper(1,i));
    } else {
      btreeset.remove(&Wrapper(memo[i],i));
      memo[i] += 1;
      btreeset.insert(Wrapper(memo[i],i));
    }
    println!("{}", btreeset.iter().next().unwrap().1 + 1);
  }
}