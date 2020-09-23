use proconio::input;
use std::collections::HashMap;

fn helper(
  map: &HashMap<usize, usize>,
  val: usize
) -> bool {
  if let Some(v) = map.get(&val) {
    if &1 < v { return false }
  }
  
  let mut i = 2;
  while i * i <= val {
    if val % i == 0 {
      if map.get(&i).is_some() || map.get(&(val / i)).is_some() {
        return false;
      }
    }
    i += 1;
  }
  true
}

fn main() {
  input! {
    n: usize,
    vals: [usize;n]
  };
   
  let mut map = HashMap::new();  
  for i in 0..n {
    let entry = map.entry(vals[i]).or_insert(0);
    *entry += 1;
  }
  
  if let Some(v) = map.get(&1) {
    if v == &1 {
      println!("1");    
    } else {
      println!("0");
    }
    return
  }
  
  let mut result = 0;
  for v in vals {
    if helper(&map, v) {
      result += 1;
    }
  }
  println!("{}", result);
}