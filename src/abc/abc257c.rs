/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    n:usize,
    s:Chars,
    vals:[isize;n]
  }

  let mut a = 0;
  let mut c = 0;
  let mut vals = vals.into_iter().enumerate().map(|(i,v)| {
    let flag = s[i] == '1';
    if flag {
      a += 1;
      (v, 1)
    } else {
      c += 1;
      (v, 0)
    }
  }).collect::<Vec<(isize,usize)>>();

  vals.sort_by(|a,b|{
    let v = a.0.cmp(&b.0);
    if v == Ordering::Equal {
      let v2 = a.1.cmp(&b.1);
      if v2 == Ordering::Greater {
        Ordering::Less
      } else {
        Ordering::Greater
      }
    } else {
      v
    }
  });

  let mut memo = vec![(vals[0].0, 0, 0)];
  for i in 0..n {
    let (v, t) = vals[i];
    let li = memo.len()-1;
    if v == vals[li].0 {
      if t == 1 {
        memo[li].1 += 1;
      } else {
        memo[li].2 += 1;
      }
    } else {
      if t == 1{
        memo.push((v, 1, 0));  
      } else {
        memo.push((v, 0, 1));
      }
    }
  }

  let mut max = 0;
  let mut ca = 0;
  let mut cc = 0;
  for (v, av, cv) in memo {
    let a_num = a - ca;
    let c_num = cc;
    max = std::cmp::max(max, a_num + c_num);
    ca += av;
    cc += cv;
  }

  max = max.max(a).max(c);

  println!("{}", max);
}