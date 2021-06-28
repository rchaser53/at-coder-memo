/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use num_complex::Complex;


const MOD:usize = 1_000_000_007;

fn co((x, y):(isize, isize)) -> Complex<isize> {
  Complex::new(x, y)
}

pub fn main(
) {
  input! {
    n:usize,
    s:[(isize,isize);n],
    t:[(isize,isize);n]
  }

  if n == 1 {
    println!("Yes");
    return
  }

  let s0 = co(s[0]);
  let s1 = co(s[1]) - s0;
  
  let s = s.into_iter().map(|p| co(p) - s0);
  let t = t.into_iter().map(co).collect::<Vec<_>>();

  for ti in &t {
    let t = t.clone().into_iter().map(|c| c - ti).collect::<Vec<_>>();
    for &tj in &t {
      if s1.norm_sqr() == tj.norm_sqr() {
        let a = s.clone().map(|v| v * tj).collect::<HashSet<_>>();
        let b = t.clone().into_iter().map(|v| v * s1).collect::<HashSet<_>>();
        if a == b {
          println!("Yes");
          return
        }
      }
    }
  }

  println!("No");
}
