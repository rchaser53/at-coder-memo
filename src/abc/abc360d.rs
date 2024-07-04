/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

type Target = isize;
type UseValue = isize;
fn lower_bound(arr: &VecDeque<Target>, x: &UseValue) -> usize {
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
    t:isize,
    s:Chars,
    x:[isize;n]
  }

  let mut x = x.into_iter().enumerate().collect::<Vec<(usize, isize)>>();
  x.sort_by(|a,b|a.1.cmp(&b.1));
  let mut ss = vec!['0';n];
  for i in 0..n {
    let (ci, _) = x[i];
    ss[i] = s[ci];
  }
  let x = x.into_iter().map(|v| v.1).collect::<Vec<isize>>();
  let s = ss;

  let mut to_minus = VecDeque::new();
  for i in 0..n {
    if s[i] == '0' {
      to_minus.push_back(x[i]);
    }
  }
  let mut result = 0;

  for i in 0..n {
    if s[i] == '1' {
      let cv = x[i];
      while !to_minus.is_empty() && cv > to_minus[0] {
        to_minus.pop_front();
      }

      let nv = cv + t * 2;
      let ti = lower_bound(&to_minus, &nv);

      if ti < to_minus.len() {
        if to_minus[ti] == nv {
          result += ti+1;
        } else {
          result += ti
        }
      } else {
        result += ti;
      }
    }
  }


  println!("{}", result);
}