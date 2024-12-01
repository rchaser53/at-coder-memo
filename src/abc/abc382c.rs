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
    n:usize,
    m:usize,
    a:[usize;n],
    b:[usize;m]
  }

  let mut memo = vec![10usize.pow(10);n];
  let mut dict = HashMap::new();
  memo[0] = a[0];

  dict.insert(memo[0], 1);

  for i in 1..n {
    let v = a[i];
    memo[i] = memo[i-1].min(v);
    if dict.get(&memo[i]).is_none() {
      dict.insert(memo[i], i+1);
    }
  }
  
  let mut memo = memo.into_iter().collect::<HashSet::<usize>>().into_iter().collect::<Vec<usize>>();
  memo.sort();

  let x = memo.len();
  for v in b {
    let ti = lower_bound(&memo, &v);
    if ti == x {
      let ti = ti - 1;
      println!("{}", dict.get(&memo[ti]).unwrap());
    } else if ti == 0 && v < memo[0] {
      println!("-1");
    } else {
      let cv = memo[ti];
      if v < cv {
        println!("{}", dict.get(&memo[ti-1]).unwrap());  
      } else {
        println!("{}", dict.get(&memo[ti]).unwrap());
      }      
    }
  }
}