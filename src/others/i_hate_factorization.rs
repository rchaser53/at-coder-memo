use proconio::input;
use std::collections::HashMap;

fn main() {
  input! {
    X: i64,
  }

  let mut map: HashMap<i64, i64> = HashMap::new();

  for a in -200i64..200i64 {
    let val = a.pow(5);
    map.insert(val, a);
  }

  for a in -200i64..200i64 {
    let val = a.pow(5);
    let diff = X - val;
    if let Some(b) = map.get(&diff) {
      println!("{} {}", a, -1 * b);
      return
    }
  }
}