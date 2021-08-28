/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;


pub fn main(
) {
  input! {
    k:usize,
    n:usize,
    mut vals:[(Chars,Chars);n]
  }

  let mut nvals = vec![];
  for i in 0..n {
    let mut nums = vec![];
    for j in 0..vals[i].0.len() {
      nums.push((vals[i].0[j] as u8 - '1' as u8) as usize);
    }
    nvals.push((nums,vals[i].1.clone()));
  }

  let limit = 4usize.pow(k as u32);
  for i in 0..limit {
    let mut f = true;
    let mut memo = vec![0;k];
    for j in 0..k {
      let mut temp = 0;
      if i >> j * 2 & 1 == 1 {
        temp += 1;
      }
      if i >> (j*2+1) & 1 == 1 {
        temp += 2;
      }
      if temp == 0 {
        f = false;
        break
      }
      memo[j] = temp;
    }

    if !f { continue }

    let mut dict = vec![None;k];
    let mut f = true;
    for j in 0..n {
      if !f { break }
      let mut tot = 0;
      for &v in &nvals[j].0 {
        tot += memo[v];
      }

      let slen = nvals[j].1.len();
      if tot != slen {
        f = false;
        break
      }

      let mut tot = 0;
      for &v in &nvals[j].0 {
        let mut temp = vec![];
        let nv = tot + memo[v];

        for k in tot..nv {
          temp.push(nvals[j].1[k]);
        }
        tot += memo[v];

        if dict[v].is_none() {
          dict[v] = Some(temp);
          continue
        }

        if let Some(mv) = &dict[v] {
          if mv != &temp {
            f = false;
            break
          }
        }
      }
    }

    if f {
      for v in dict {
        if let Some(v) = v {
          println!("{}", v.iter().map(|v| v.to_string()).collect::<String>());
        }
      }
      return
    }
  }
}
