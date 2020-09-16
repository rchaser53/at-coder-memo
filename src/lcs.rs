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