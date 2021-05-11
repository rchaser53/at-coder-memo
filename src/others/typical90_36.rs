/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

pub fn main(
) {
    input! {
      n:usize,
      q:usize,
      mut vals:[(isize,isize);n],
      queries:[Usize1;q]
    }

    vals = vals.into_iter().map(|v| (v.0 - v.1, v.0 + v.1)).collect();

    let mut vals_x = vals.clone();
    let mut vals_y = vals.clone();
    vals_x.sort_by(|a,b| a.0.cmp(&b.0));
    vals_y.sort_by(|a,b| a.1.cmp(&b.1));

    for i in queries {
      let mut max = std::cmp::max(
        (vals[i].0 - vals_x[0].0).abs(),
        (vals[i].0 - vals_x[vals_x.len()-1].0).abs()
      );
      max = std::cmp::max(max, (vals[i].1 - vals_y[0].1).abs());
      max = std::cmp::max(max, (vals[i].1 - vals_y[vals_y.len()-1].1).abs());
      println!("{}", max);
    }
}
