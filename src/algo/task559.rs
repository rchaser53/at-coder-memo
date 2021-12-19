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

fn find(memo:&mut Vec<isize>, a:isize) -> isize {
  let pi = memo[a as usize];
  if pi < 0 {
    a
  } else {
    let v = find(memo, pi);
    memo[a as usize] = v;
    memo[a as usize]
  }
}

fn connect(memo: &mut Vec<isize>, a:isize, b:isize) -> bool {
  let mut api = find(memo, a);
  let mut bpi = find(memo, b);
  if api == bpi { return false }

  let apv = memo[api as usize] * -1;
  let bpv = memo[bpi as usize] * -1;
  if api < bpi {
    std::mem::swap(&mut api, &mut bpi);
  }

  memo[api as usize] += memo[bpi as usize];
  memo[bpi as usize] = api;
  true
}

fn main() {
    let vals: Vec<usize> = readvec();
    let (n,m) = (vals[0], vals[1]);
    let mut memo = vec![-1;n];
    for _ in 0..m {
      let vals: Vec<isize> = readvec();
      let (a, b) = (vals[0], vals[1]);
      if connect(&mut memo, a, b) {
        println!("No");
      } else {
        println!("Yes");
      }
    }
}
