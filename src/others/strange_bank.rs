use proconio::input;

const INF: usize = 1_000_000;

struct Helper {
  pub n: usize,
  pub memo: Vec<usize>
}

impl Helper {
  pub fn new(n: usize) -> Helper {
    Helper {
      n: n,
      memo: vec![INF;n+1]
    }
  }
  
  pub fn traverse(&mut self, left: usize) -> usize {
    if left == 0 {
      return 0
    }

    if self.memo[left] != INF {
      return self.memo[left]
    }
    let mut res = INF;

    let mut nine = 1;
    while nine <= left {
      res = std::cmp::min(self.traverse(left - nine) + 1, res);
      nine *= 9;
    }
    
    let mut six = 1;
    while six <= left {
      res = std::cmp::min(self.traverse(left - six) + 1, res);
      six *= 6;
    }

    self.memo[left] = res;
    res
  }
}

fn main() {
  input! {
    n: usize,
  }

  let mut helper = Helper::new(n);  
  println!("{}", helper.traverse(n));
}

use proconio::input;

fn main() {
  input! {
    n: usize,
  }
  
  let max_value = 100_000;
  let mut dp: Vec<usize> = vec![max_value+1;max_value+1];
  let mut choices: Vec<usize> = vec![1];
  dp[1] = 1;
  
  let mut v = 6;
  while v <= max_value {
    dp[v] = 1;
    choices.push(v);
    v *= 6;
  }

  let mut v = 9;
  while v <= max_value {
    dp[v] = 1;
    choices.push(v);
    v *= 9;
  }
  
  for i in 2..=max_value {
    for v in choices.iter() {
      if &i < v { continue }
      let mut temp = std::cmp::min(dp[i-v]+1, dp[i-1]+1);
      dp[i] = std::cmp::min(dp[i], temp);
    }
  }
  
  println!("{}", dp[n]);
}