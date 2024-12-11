/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    h:usize,
    w:usize,
    d:i32,
    rows:[Chars;h]
  }

  let mut p = vec![];
  for i in -d..=d {
    for j in -d..=d {
      if i.abs() + j.abs() <= d {
        p.push((i,j));
      }
    }
  }
  
  let hi = h as i32;
  let wi = w as i32;

  let mut memo = vec![];
  for i1 in 0..h {
    for j1 in 0..w {
      if rows[i1][j1] == '#' { continue }
      let ii = i1 as i32;
      let ji = j1 as i32;

      let mut set = HashSet::new();
      for &(ai,aj) in &p {
        let ni = ii+ai;
        let nj = ji+aj;
        if ni < 0 || hi <= ni { continue }
        if nj < 0 || wi <= nj { continue }
        if rows[ni as usize][nj as usize] == '#' { continue }

        set.insert((ni,nj));
      }
      memo.push(set);
    }
  }

  let mut result = 0;
  let n = memo.len();
  for i in 0..n {
    for j in 0..n {
      if i == j { continue }
      let mut temp = memo[i].len();
      for v in &memo[j] {
        if !memo[i].contains(v) {
          temp += 1;
        }
      }

      result = result.max(temp);
    }
  }

  println!("{}", result);

}