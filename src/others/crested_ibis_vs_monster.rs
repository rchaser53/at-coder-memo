use proconio::input;
use std::collections::HashMap;

fn main() {
  input! {
    h: usize,
    n: usize,
    mut ab: [(usize, usize);n]
  }
  let mut magics: HashMap<usize, usize> = HashMap::new();
  
  for i in 0..n {
    let (a, b) = ab[i];
    if let Some(v) = magics.get(&a) {
      magics.insert(a, std::cmp::min(*v, b));
    } else {
      magics.insert(a, b);
    }
  }
  
  let mut dp = vec![10usize.pow(10); h+1];
  dp[0] = 0;
  for i in 0..h {
    let v = dp[i];
    for (a, m) in magics.iter() {
      let next_index = if i + a > h {
        h
      } else {
        i + a
      };
      
      dp[next_index] = std::cmp::min(dp[next_index], m + v);
    }
  }

  println!("{}", dp[h]);
}
