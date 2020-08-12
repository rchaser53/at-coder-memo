use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

impl PartialEq for Wrapper {
  fn eq(&self, other: &Wrapper) -> bool {
    self.0 == other.0
  }
}
impl Eq for Wrapper {}

impl PartialOrd for Wrapper {
  fn partial_cmp(&self, other: &Wrapper) -> Option<Ordering> {
    let result = if self.0 < other.0 {
      Ordering::Less
    } else if self.0 == other.0 {
      Ordering::Equal
    } else {
      Ordering::Greater
    };
    Some(result)
  }
}

impl Ord for Wrapper {
  fn cmp(&self, other: &Wrapper) -> Ordering {
    if self.0 < other.0 {
      Ordering::Less
    } else if self.0 == other.0 {
      Ordering::Equal
    } else {
      Ordering::Greater
    }
  }
}

#[derive(Debug)] 
struct Wrapper(f64);

fn main() {
  input! {
    n: usize,
    m: usize,
    mut products: [f64;n] 
  }
  let mut heap = BinaryHeap::new();
  for v in products {
    heap.push(Wrapper(v));
  }

  for _ in 0..m {
    let mut v = heap.pop().unwrap();
    v.0 /= 2f64; 
    heap.push(v);
  }
  
  let mut sum = 0;
  for v in heap.into_iter() {
    sum += v.0.trunc() as usize;
  }
  
  println!("{}", sum);
}