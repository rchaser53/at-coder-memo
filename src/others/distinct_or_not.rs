use proconio::input;
use std::collections::HashMap;

fn main() {
  input! {
    n: usize,
    a: [usize;n]
  }
  let mut memo: HashMap<usize, bool> = HashMap::new();
  
  for v in a {
    if memo.contains_key(&v) {
      println!("NO");
      return
    } else {
      memo.insert(v, true);
    }
  }
  println!("YES");
}
  