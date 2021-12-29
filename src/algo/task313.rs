/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;

fn readln<T: std::str::FromStr>() -> T {
  let mut tmp = String::new();
  std::io::stdin().read_line(&mut tmp).ok();
  tmp.trim().parse().ok().unwrap()
}
fn readvec<T: std::str::FromStr>() -> Vec<T> {
  readln::<String>()
      .split_whitespace()
      .map(|x| x.parse().ok().unwrap())
      .collect()
}
fn read_chars() -> Vec<char> {
  let mut tmp = String::new();
  std::io::stdin().read_line(&mut tmp).ok();
  let tmp:String = tmp.trim().parse().ok().unwrap();
  tmp.chars().into_iter().collect::<Vec<char>>()
}

const INF:usize = 1_000_000_000_000usize;
fn main() {
  let dict:Vec<usize> = readvec();
  let (n, m) = (dict[0], dict[1]);
  let mut dict:Vec<Vec<usize>> = vec![];
  for _ in 0..n {
    dict.push(readvec());
  }
  
  let mut memo = vec![INF;m+1];
  memo[0] = 0;

  for i in 0..n {
    let mut new_memo = vec![INF;m+1];
    for j in 0..=m {
      if memo[j] < INF {
        new_memo[j] = 0;
      }

      let (av, bv) = (dict[i][0], dict[i][1]);
      if j < av { continue }
      let ni = j - av;
      if memo[ni] < INF {
        new_memo[j] = std::cmp::min(new_memo[j], 1);
      }
      if new_memo[ni] < bv {
        new_memo[j] = std::cmp::min(new_memo[j], new_memo[ni]+1);
      }
    }

    memo = new_memo;
  }

  if memo[m] == INF {
    println!("No");
  } else {
    println!("Yes");
  }
}