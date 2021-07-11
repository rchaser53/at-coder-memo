/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;

fn rerooting<
  T: Clone,
  E: IntoIterator<Item = (usize, usize)>,
  F: FnMut(T, T) -> T,
  G: FnMut(T, usize) -> T,
> (
  n: usize,
  edges: E,
  identity: T,
  mut operate: F,
  mut operate_node: G
) -> Vec<T> {
  const NO_PARENT:usize = std::usize::MAX;

  let mut g = vec![vec![];n];
  let mut i_g = vec![vec![];n];
  for (u, v) in edges {
    i_g[u].push(g[v].len());
    i_g[v].push(g[u].len());
    g[u].push(v);
    g[v].push(u);
  }

  if n == 0 {
    return vec![]
  }

  if n == 1 {
    return vec![operate_node(identity, 0)];
  }

  let mut parents = vec![0;n];
  let mut order = vec![0;n];

  // initialize orderd tree
  {
    let mut index = 0;
    let mut stack = vec![0];
    parents[0] = NO_PARENT;
    while let Some(ci) = stack.pop() {
      order[index] = ci;
      index += 1;

      for i in 0..g[ci].len() {
        let ni = g[ci][i];
        if ni == parents[ci] { continue }
        stack.push(ni);
        parents[ni] = ci;
      }
    }
  }

  let mut dp = (0..n)
    .map(|i| vec![identity.clone(); g[i].len()])
    .collect::<Vec<_>>();
  
  // from leaf
  for i in (1..n).rev() {
    let ci = order[i];
    let parent = parents[ci];
    
    let mut accum = identity.clone();
    let mut pi = NO_PARENT;

    for j in 0..g[ci].len() {
      if g[ci][j] == parent {
        pi = j;
        continue
      }
      accum = operate(accum, dp[ci][j].clone());
    }
    let ti = i_g[ci][pi];
    dp[parent][ti] = operate_node(accum, ci);
  }

  let mut result = vec![identity.clone();n];
  let mut accums_from_tail = vec![];

  // to leaf
  for i in 0..n {
    let ci = order[i];
    let len = g[ci].len();
    let mut accum = identity.clone();

    accums_from_tail.clear();
    accums_from_tail.extend(
      std::iter::repeat(identity.clone()).take(len)
    );

    for j in (1..len).rev() {
      accums_from_tail[j-1] = operate(accums_from_tail[j].clone(), dp[ci][j].clone());
    }

    for j in 0..len {
      let ti = i_g[ci][j];
      dp[g[ci][j]][ti] = operate_node(
        operate(accum.clone(), accums_from_tail[j].clone()), ci
      );
      accum = operate(accum, dp[ci][j].clone());
    }
    result[ci] = operate_node(accum, ci);
  }

  result
}

fn pow(x:usize, n:usize) -> usize {
  let (mut x, mut y, mut n) = (x % MOD, 1usize, n);
  while 0 < n {
    if n % 2 != 0 {
      y = (y * x) % MOD;
      n -= 1;
    }

    x = (x * x) % MOD;
    n /= 2;
  }
  y
}

// 部分木に対して持つ計算結果
#[derive(Clone, Copy)]
struct Helper {
  combo: usize,     // 場合の数
  size: usize,      // 超点数
  divisor: usize    // (size!)^(-1)
}

impl Helper {
  fn empty() -> Helper {
    Helper {
      combo: 1,
      size: 0,
      divisor: 1
    }
  }

  fn append(self, other:Helper) -> Helper {
    Helper {
      combo: self.combo * other.combo % MOD,
      size: self.size + other.size,
      divisor: self.divisor * other.divisor % MOD
    }
  }
}


pub fn main(
) {
  input! {
    n:usize,
    edges:[(Usize1,Usize1);n-1]
  }
  
  let m = n+1;
  let mut fact = vec![0;m];
  fact[0] = 1;
  for i in 1..m {
    fact[i] = fact[i-1] * i % MOD;
  }

  let mut fact_inv = vec![0;m];
  fact_inv[m-1] = pow(fact[m-1], MOD-2);
  for i in (0..m-1).rev() {
    fact_inv[i] = fact_inv[i+1] * (i+1) % MOD;
  }

  let operate_node = |x: Helper, _u:usize| -> Helper {
    // x:各部分木に対するoperate_nodeの結果をX::appendで合成したもの
    Helper {
      combo: fact[x.size] * x.combo % MOD * x.divisor % MOD,
      size: x.size + 1,
      divisor: fact_inv[x.size+1]
    }
  };

  let result = rerooting(
    n,
    edges.into_iter(),
    Helper::empty(),
    Helper::append,
    operate_node
  );

  for i in 0..n {
    println!("{}", result[i].combo);
  }
}
