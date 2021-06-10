/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    h:usize,
    w:usize,
    vals:[[usize;w];h],
  }

  let mut map = HashMap::new();
  for i in 0..w {
    let mut smap = HashMap::new();
    for j in 0..h {
      *smap.entry(vals[j][i]).or_insert(0) += 1 << j;
    }

    for (key, v) in smap {
      let base_entry = map.entry(key).or_insert(vec![]);
      base_entry.push(v);
    }
  }

  let limit = 1 << h;
  let mut memo = vec![0;limit];
  for i in 0..limit {
    let mut temp = 0;
    for j in 0..h {
      if i >> j & 1 == 1 {
        temp += 1;
      }
    }
    memo[i] = temp;
  }

  let mut result = 0;
  for (_, smap) in map {
    for i in 0..limit {
      let mut count = 0;
      for &v in &smap {
        if v & i == i {
          count += 1;
        }
      }
      result = std::cmp::max(result, memo[i] * count);
    }
  }
  println!("{}", result);
}