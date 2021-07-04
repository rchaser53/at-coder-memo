/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn create_map(
  vals: &Vec<usize>,
) -> HashMap<usize, usize> {
  let mut map = HashMap::new();
  for &v in vals {
    *map.entry(v % 46).or_insert(0) += 1;
  }
  map
} 

pub fn main(
) {
    input! {
      n:usize,
      aa:[usize;n],
      bb:[usize;n],
      cc:[usize;n],
    }

    let ma = create_map(&aa);
    let mb = create_map(&bb);
    let mc = create_map(&cc);

    let mut count = 0;
    for i in 0..46 {
      let mut temp = vec![0,0,0];
      if let Some(v) = ma.get(&i) {
        temp[0] = *v;
      } else {
        continue
      }

      for j in 0..46 {
        if let Some(v) = mb.get(&j) {
          temp[1] = *v;
        } else {
          continue
        }

        for k in 0..46 {
          if let Some(v) = mc.get(&k) {
            temp[2] = *v;
          } else {
            continue
          }

          if (i+j+k) % 46 == 0 {
            count += temp[0] * temp[1] * temp[2];
          }
        }
      }
    }
    println!("{}", count);
}
