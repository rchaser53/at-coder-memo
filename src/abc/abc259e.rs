/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
  }

  let mut map = HashMap::new();
  let mut data = vec![vec![];n];
  for i in 0..n {
    input! {
      m:usize,
      vals:[(usize,usize);m]
    }

    for &(v, num) in &vals {
      let entry = map.entry(v).or_insert((num, 0));

      if entry.0 == num {
        entry.1 += 1;
      } else if entry.0 < num {
        *entry = (num, 1);
      }
    }
    data[i] = vals;
  }

  let mut result = 0;
  let mut seen = false;
  for i in 0..n {
    let mut success = false;
    for &(v, cv1) in &data[i] {
      if let Some((cv2, num)) = map.get(&v) {
        if cv1 == *cv2 && *num == 1 {
          success = true;
        } 
      }
    }

    if success {
      result += 1;
    } else {
      seen = true;
    }
  }

  if seen {
    result += 1;
  }

  println!("{}", result);
}