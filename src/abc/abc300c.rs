/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input!{
    h:usize,
    w:usize,
    rows:[Chars;h]
  }
  
  let n = h.min(w);
  let mut result: VecDeque<usize> = vec![0;n+1].into_iter().collect::<_>();

  for i in 1..h-1 {
    for j in 1..w-1 {
      if rows[i][j] == '.' { continue }
      if rows[i-1][j-1] == '.' { continue }
      if rows[i-1][j+1] == '.' { continue }
      if rows[i+1][j-1] == '.' { continue }
      if rows[i+1][j+1] == '.' { continue }

      let mut count = 1;
      for _ in 0.. {
        let pad = count + 1;
        
        let a = pad <= i && pad <= j && rows[i-pad][j-pad] == '#';
        let b = pad <= i && j+pad < w && rows[i-pad][j+pad] == '#';
        let c = i+pad < h && pad <= j && rows[i+pad][j-pad] == '#';
        let d = i+pad < h && j+pad < w && rows[i+pad][j+pad] == '#';

        if a && b && c && d {
          count = pad;
        } else {
          result[count] += 1;
          break
        }
      }
    }
  }

  result.pop_front();
  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" "));
}