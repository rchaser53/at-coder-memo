/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    h:usize,
    w:usize,
    s:[Chars;h]
  }

  let ih = h as isize;
  let iw = w as isize;
  let p = [
    [0,0,0,0],
    [1,2,3,4],
    [-1,-2,-3,-4]
  ];
  let dict = ['n', 'u', 'k', 'e'];
  
  for i in 0..ih {
    for j in 0..iw {
      if s[i as usize][j as usize] != 's' { continue }

      for pi in 0..3 {
        for pj in 0..3 {
          if pi == 0 && pj == 0 { continue }

          let ni = i + p[pi][3];
          let nj = j + p[pj][3];

          if ni < 0 || ih <= ni || nj < 0 || iw <= nj { continue }

          let mut success = true;
          for k in 0..4 {
            let di = p[pi][k] ;
            let dj = p[pj][k];

            let ai = (i+di) as usize;
            let aj = (j+dj) as usize;
            if s[ai][aj] != dict[k] {
              success = false;
              break
            }
          }

          if success {
            println!("{} {}", i+1, j+1);
            for k in 0..4 {
              let di = p[pi][k] ;
              let dj = p[pj][k];
  
              let ai = (i+di) as usize + 1;
              let aj = (j+dj) as usize + 1;
              println!("{} {}", ai, aj);
            }
            return
          }
        }
      }
    }
  }

}