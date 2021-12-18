/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
  use std::cmp::Reverse;
  
  fn readln<T: std::str::FromStr>() -> T {
    let mut tmp = String::new();
    std::io::stdin().read_line(&mut tmp).ok();
    tmp.trim().parse().ok().unwrap()
  }
  
  fn readvec<T: std::str::FromStr>() -> Vec<T> {
    readln::<String>()
        .split_whitespace()
        .map(|x| x.parse().ok().unwrap())
        .collect()
  }
  
  fn helper(n:usize,l:usize,r:usize) -> Vec<VecDeque<usize>> {
    if r < l {
      return vec![]
    }
    if n == 0 {
      return vec![VecDeque::new()]
    }

    let mut result = vec![];
    let temp = helper(n-1,l,r);
    for mut v in temp {
      v.push_front(l);
      result.push(v);
    }

    let temp = helper(n,l+1,r);
    for v in temp {
      result.push(v);
    }

    result
  }

  fn main() {
    let vals: Vec<usize> = readvec();
    let (l,r) = (vals[0], vals[1]);

    let memo = helper(10, 0, 9);
    let mut memo = memo.into_iter().map(|v| {
      let mut v = v.into_iter().collect::<Vec<usize>>();
      v.reverse();
      let mut result = 0;
      for i in 0..v.len() {
        result += 10usize.pow(i as u32) * v[i];
      }
      result
    }).collect::<Vec<usize>>();
    memo.sort();

    let mut lv = 0;
    let mut same = false;
    for &v in &memo {
      if l < v {
        break
      } else if l == v{
        same = true;
      }
      lv += v;
    }

    let mut rv = 0;
    for &v in &memo {
      if r < v {
        break
      }
      rv += v;
    }

    let mut result = rv - lv;
    if same {
      result += l;
    }

    println!("{}", result);
  }