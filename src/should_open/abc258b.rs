/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;


fn culc(mut arr: Vec<usize>, n:usize) -> usize {
  arr.reverse();
  let mut max = 0;
  for i in 0..=n {
    let mut temp = 0;
    for j in 0..n {
      temp += 10usize.pow(j as u32) * arr[(i+j) % n];
    }
    max = std::cmp::max(max, temp);
  }
  max
} 

// grid ループ 斜め 移動 bfs
const XAXIS:[isize;3] = [0, 1, -1];
const YAXIS:[isize;3] = [0, 1, -1];
fn helper<T: Copy>(
  arr: &Vec<Vec<T>>, ci:usize, cj:usize, i:isize, j:isize,
  enable_loop: bool // ループを許可する場合はtrue
) -> Option<T> {
  let n = arr.len();
  let m = arr[0].len();
  let inn = n as isize;
  let im = m as isize;

  let ni = ci as isize + i;
  let nj = cj as isize + j;

  if !enable_loop {
    if ni < 0 || inn <= ni || nj < 0 || im <= nj {
      return None
    }
  }

  let inner_helper = |limit:isize, bv:isize, av:isize| -> usize {
    let nv = bv + av;
    if 0 <= nv {
      (nv % limit) as usize
    } else {
      (limit + nv) as usize
    }
  };

  let ti = inner_helper(inn, ci as isize, i);
  let tj = inner_helper(im, cj as isize, j);
  Some(arr[ti][tj])
}

fn main() {
  input! {
    n:usize,
    bvals:[Chars;n]
  }
 
  let mut vals = vec![vec![0;n];n];
  for i in 0..n {
    for j in 0..n {
      vals[i][j] = (bvals[i][j] as u8 - '0' as u8) as usize;
    }
  }
 
  let mut result = 0;
  let inn = n as isize;
  for i in 0..n {
    for j in 0..n {
      for &y in &YAXIS {
        for &x in &XAXIS {
          // x:0 y:0だと移動しないのでcontinue
          if x == y && x == 0 && y == 0 { continue }
          let mut arr = vec![0;n];

          for k in 0..inn {
            if let Some(v) = helper(&vals, i, j, y * k, x * k, true) {
              arr[k as usize] = v;
            }
          }
          let v = culc(arr, n);
          result = std::cmp::max(v, result);
        }
      }
    }
  }
 
  println!("{}", result)
}