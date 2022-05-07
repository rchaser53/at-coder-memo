use std::collections::*;
const MOD:usize = 1_000_000_007;

fn mod_pow(mut a: usize, mut n: usize) -> usize {
  let mut res = 1;
  while n > 0 {
    if n & 1 == 1 {
      res = res * a % MOD; 
    }
    a = a * a % MOD;
    n >>= 1;
  }
  res
}

// nCk (choose)とかでnの値が変動するケースの際に直接使う羽目になる
fn mod_inv(a: usize) -> usize {
  mod_pow(a, MOD-2)
}

pub fn directly_using_choose(queries: Vec<Vec<i32>>) -> Vec<i32> {
  let mut result = vec![];

  for arr in queries {
    let n = arr[0] as usize;
    let mut k = arr[1] as usize;

    let mut i = 2;
    let mut map = HashMap::new();
    while i * i <= k {
      if k % i == 0 {
        *map.entry(i).or_insert(0) += 1;
        k /= i;
      } else {
        i += 1;
      }
    }
    if 1 < k {
      *map.entry(k).or_insert(0) += 1;
    }
    
    let mut temp = 1;
    for (_, choose_k) in map {
      let choose_n = n + choose_k - 1;

      let mut cv = 1;
      let mut pv = 1;
      for i in (choose_n-choose_k+1)..=choose_n {
        cv *= i;
        cv %= MOD;
      }
      for i in 1..=choose_k {
        pv *= i;
        pv %= MOD;
      }

      // % MODを忘れないように
      temp *= cv * mod_inv(pv) % MOD;
      temp %= MOD;
    }

    result.push(temp as i32);
  }  
  result
}
