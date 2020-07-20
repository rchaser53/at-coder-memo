use proconio::input;
use std::collections::HashMap;
 
fn main() {
  input! {
    N: usize,
    A: [usize;N]
  }
  let mut map: HashMap<usize, usize> = HashMap::new();
  for v in A.iter() {
    let mut a = map.entry(*v).or_insert(0);
    *a += 1;
  }
  
  let mut total = 0;
  
  for key in map.keys() {
    let val = map.get(&key).unwrap();
    if val > &1 {
      total += val * (val - 1) / 2;
    }
  }
  
  for i in A {
    if let Some(v) = map.get(&i) {
      if v > &1 {
        let before = v * (v-1) /2;
        let after = (v-1) * (v-2) / 2;
        println!("{}", total - before + after);
      } else {
        println!("{}", total);
      }
    } else {
      println!("{}", 0);
    }
  }
}