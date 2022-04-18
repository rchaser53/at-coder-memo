fn test(need: &Vec<i32>, memo: &Vec<i32>) -> bool {
  for i in 0..4 {
    if memo[i] < need[i] {
      return false
    }
  }
  true
}

// 連続した部分文字列を一度だけ置き換えてQ,W,E,Rが全て同じ文字数にする
// 連続した部分文字列の最短を求めよ
pub fn syakutori(s: String) -> i32 {
  let n = s.len();
  let s = s.chars().collect::<Vec<char>>();
  let mut memo = vec![vec![0;4];n+1];
  for i in 0..n {
    let c = s[i];

    for j in 0..4 {
      memo[i+1][j] = memo[i][j];
    }

    match c {
      'Q' => memo[i+1][0] += 1,
      'W' => memo[i+1][1] += 1,
      'E' => memo[i+1][2] += 1,
      _ => memo[i+1][3] += 1,
    }
  }
  
  let bv = memo[n].iter().sum::<i32>() / 4;
  let mut need = vec![0;4];
  for i in 0..4 {
    let v = memo[n][i];
    if bv < v {
      need[i] = v - bv;
    }
  }

  if test(&need, &vec![0;4]) {
    return 0
  }

  let mut result = n;
  let mut left = 0;
  let mut right = 0;
  let mut memo = vec![0;4];
  while left < n && right < n {
    while right < n && !test(&need, &memo) {
      let c = s[right];
      match c {
        'Q' => memo[0] += 1,
        'W' => memo[1] += 1,
        'E' => memo[2] += 1,
        _ => memo[3] += 1,
      }
      right += 1;
    }

    // ここから次のループまでrightが1多い
    if test(&need, &memo) {
      result = std::cmp::min(right - left, result);
    }

    while left < right && test(&need, &memo) {
      result = std::cmp::min(right-left, result);
      let c = s[left];
      match c {
        'Q' => memo[0] -= 1,
        'W' => memo[1] -= 1,
        'E' => memo[2] -= 1,
        _ => memo[3] -= 1,
      }
      left += 1;
    }
  }
  
  result as i32
}