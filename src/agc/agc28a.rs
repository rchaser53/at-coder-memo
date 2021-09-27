use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn gcd(a: usize, b: usize) -> usize {
  if b == 0 {
    return a
  }
  gcd(b, a % b)
}

fn lcm(a: usize, b: usize) -> usize {
  a * b / gcd(a, b)
}

fn main() {
  input!{
    n:usize,
    m:usize,
    s:Chars,
    t:Chars,
  }

  let lcmv = lcm(n, m);
  let nv = lcmv / n;
  let mv = lcmv / m;
  
  let mut smap = HashMap::new();
  let mut tmap = HashMap::new();
  for i in 0..n {
    smap.insert(i*nv, s[i]);
  }
  for i in 0..m {
    tmap.insert(i*mv, t[i]);
  }

  for (key, &v1) in &smap {
    if let Some(&v2) = tmap.get(key) {
      if v1 != v2 {
        println!("-1");
        return
      }
    }
  }

  println!("{}", lcmv);
}