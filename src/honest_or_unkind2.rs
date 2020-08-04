use proconio::input;
 
fn main() {
  input! {
    n: usize,
  }
  let mut patterns:Vec<Vec<usize>> = vec![vec![10;n];n];
  for i in 0..n {
    input! {
      p: usize,
    }
    
    for _ in 0..p {
      input! {
        a: usize,
        b: usize
      }
      patterns[i][a-1] = b;
    }
  }
 
  let max = 2usize.pow(n as u32);
  let mut result = 0;
  for i in 1..=max {
    let mut memo = vec![0;n];
    let mut count = 0;
    for index in 0..n {
      if i>>index & 1 == 1 {
        memo[index] = 1;
        count += 1;
      }
    }
    
    let mut success = true;
    for index in 0..n {
      if memo[index] != 1 {
        continue
      }
 
      for (i, v) in patterns[index].iter().enumerate() {
        if *v == 10 { continue }
        if memo[i] != *v {
          success = false;
          break;
        }
      }
      if !success {
        break
      }
    }
 
    if success {
      result = std::cmp::max(result, count);
    }
  }
  println!("{}", result);
}