use proconio::{input, fastout};
use std::collections::HashMap;

#[fastout]
fn main() {
  input! {
    n: usize,
    vals: [isize;n]
  }
  let mut result = 0;
  
  let mut map: HashMap<isize, isize> = HashMap::new();
  
  for i in 2..n {
    let v = vals[i];
    let i = i as isize;
    let entry = map.entry(i - v).or_insert(0);
    *entry += 1;
  }
  for i in 0..n {
    let v = vals[i];
    let i = i as isize;
    if let Some(v) = map.get(&(v+i)) {
      result += v;
    }
  }
  println!("{}", result);
}