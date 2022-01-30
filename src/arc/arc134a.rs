/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    n:usize,
    l:usize,
    w:usize,
    vals:[usize;n]
  }

  let mut now = 0;
  let mut result = 0;
  let mut vals = vals.into_iter().collect::<VecDeque<usize>>();
  loop {
    if l <= now {
      println!("{}", result);
      return
    }

    if let Some(av) = vals.pop_front() {
      if av <= now && now < av + w {
        now = av + w;
      } else if av + w <= now {
      } else if now < av {
        let mut temp = (av - now) / w;
        if (av - now) % w != 0 {
          temp += 1;
        }
        result += temp;
        now = av + w;
      }
    } else {
      let mut temp = (l - now) / w;
      if (l-now) % w != 0 {
        temp += 1;
      }
      println!("{}", result + temp);
      return
    }
  }

}