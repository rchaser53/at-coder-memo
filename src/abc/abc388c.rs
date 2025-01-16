/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

type Target = usize;
type UseValue = usize;
fn lower_bound(arr: &Vec<Target>, x: &UseValue) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    while low != high {
        let mid = (low + high) / 2;
        // NEEDS TO EDIT
        match arr[mid].cmp(x) {
            std::cmp::Ordering::Less => {
                low = mid + 1;
            }
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                high = mid;
            }
        }
    }
    low
}

fn main() {
  input! {
    mut n:usize,
    mut a:[usize;n]
  }
  a.sort();

  let mut result = 0;
  for i in 0..n-1 {
    let v = a[i] * 2;
    let ti = lower_bound(&a, &v);
    result += n - ti;    
  }
  println!("{}", result);

}