#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};
 
#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        c: [Chars;h],
  	}
  
  	let mut result = 0;
  	for row_mask in 0..(1 << h) {
      for column_mask in 0..(1 << w) {
          let mut count = 0;
          for row in  0..h {
              for column in 0..w {
                if ((row_mask >> row) & 1) == 0 && ((column_mask >> column) & 1) == 0 && c[row][column] == '#' {
                  count += 1;
                }
              }
          }

          if count == k {
            result += 1;
          }
      }
  	}
	println!("{}", result);
}