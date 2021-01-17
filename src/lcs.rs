use proconio::input;
 
fn main() {
  input! {
    s: String,
    t: String,
  }
  
  let s = s.chars().collect::<Vec<char>>();
  let t = t.chars().collect::<Vec<char>>();
  
  let mut dp:Vec<Vec<usize>> = vec![vec![0;s.len()+1];t.len()+1];
  
  for i in 1..=t.len() {
    for ii in 1..=s.len() {
      let tv = t[i-1];
      let sv = s[ii-1];
      if tv == sv {
        dp[i][ii] = dp[i-1][ii-1] + 1;
      } else {
        dp[i][ii] = std::cmp::max(dp[i-1][ii], dp[i][ii-1]);
      }
    }
  }
    
  let mut r = t.len();
  let mut c = s.len();
  if dp[r][c] == 0 {
    println!("");
    return
  }
  
  let mut len = dp[r][c];
  let mut result:Vec<char> = vec!['a';len];
  while 0 < len {
    if t[r-1] == s[c-1] {
      result[len-1] = s[c-1];
      r -= 1;
      c -= 1;
      len -= 1;
    } else if dp[r][c] == dp[r-1][c] {
      r -= 1;
    } else {
      c -= 1;
    }
  }
 
  println!("{}", result.iter().collect::<String>());
}


fn my_answer() {
  input!{
    s: Chars,
    t: Chars,
  }
  
  let mut dp = vec![vec![0;s.len()];t.len()];
  if s[0] == t[0] {
    dp[0][0] = 1;
  }
  
  for i in 1..s.len() {
    if s[i] == t[0] {
      dp[0][i] = 1;
    } else {
      dp[0][i] = dp[0][i-1];
    }
  }
  for i in 1..t.len() {
    if t[i] == s[0] {
      dp[i][0] = 1;
    } else {
      dp[i][0] = dp[i-1][0];
    }
  }
  
  for i in 1..t.len() {
    for ii in 1..s.len() {
      dp[i][ii] = std::cmp::max(dp[i-1][ii], dp[i][ii-1]);
      if t[i] == s[ii] {
        dp[i][ii] = std::cmp::max(dp[i][ii], dp[i-1][ii-1]+1);
      }
    }
  }
  
  let mut result = vec![];
  let mut r = t.len()-1;
  let mut c = s.len()-1;
 
  while 0 < r && 0 < c {
    if dp[r][c] == dp[r-1][c] {
      r -= 1;
    } else if dp[r][c] == dp[r][c-1] {
      c -= 1;
    } else {
      result.push(s[c].to_string());
      r -= 1;
      c -= 1;
    }
  }
 
  if r == 0 && c == 0 {}
  else if r == 0 {
    while 0 < c {
      if dp[r][c-1] < dp[r][c] {
        result.push(s[c].to_string());
      }
      c -= 1;    
    }
  } else if c == 0 {
    while 0 < r {
      if dp[r-1][c] < dp[r][c] {
        result.push(t[r].to_string());
      }
      r -= 1;    
    }
  }
    
  if dp[0][0] == 1 {
    result.push(s[0].to_string());
  }
  
  result.reverse();
  println!("{}", result.into_iter().collect::<String>());
}
