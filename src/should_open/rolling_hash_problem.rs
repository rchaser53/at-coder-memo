use std::collections::*;

// 各文字の出現回数が一致する連続部分文字列はいくつあるか？
// 文字の範囲は0-9まで。ex. Q:1212 A:5
// ローリングハッシュ (rolling hash)
const MOD:usize = 1_000_000_007;
pub fn rolling_hash_problem(s: String) -> usize {
  let n = s.len();
  let s = s.chars().collect::<Vec<char>>();
  let s = s.into_iter().map(|v| (v as u8 - '0' as u8) as usize).collect::<Vec<usize>>();

  let mut set = HashSet::new();
  for i in 0..n {

    let value_pattern = 10; // 0-9
    let mut memo = vec![0;value_pattern];
    let mut hash = 0usize;
    for j in i..n {
      let digit = s[j];
      memo[digit] += 1;
      // 0だと空文字列になってしまうので,1加えている
      hash = (hash * (value_pattern+1) + digit+1) % MOD;

      let a = memo[digit];
      let mut success = true;
      for &v in &memo {
        if v != 0 && a != v {
          success = false;
          break
        }
      }
      if success {
        set.insert(hash);
      }
    }
  }

  set.len()
}