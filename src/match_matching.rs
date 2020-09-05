use proconio::input;
use std::collections::HashMap;

fn max(a: &str, b:&str) -> bool {
  if a.len() == b.len() {
    a > b
  } else {
    a.len() > b.len()
  }
}

fn main() {
  input! {
    n: usize,
    m: usize,
    mut vals: [usize;m]
  }

  let mut dp: Vec<String> = vec![String::from("");n+1];
  let mut map: HashMap<usize, usize> = HashMap::new();
  let def: [usize;9] = [2, 5, 5, 4, 5, 6, 3, 7, 6];
  let mut index = 1;
  for v in def.into_iter() {
    map.insert(index, *v);
    index += 1;
  }
  
  let vals: Vec<(String, usize)> = vals
  .into_iter()
  .map(|v| (v.to_string(), *map.get(&v).unwrap()))
  .collect();
  
  for v in vals.iter() {
    if n+1 <= v.1 { continue }
    let next = v.0.to_string();
    if max(&next, &dp[v.1]) {
      dp[v.1] = next;
    }
  }
  
  for current in 0..=n {
    for (num, cost) in vals.iter() {
      let cost = *cost;
      if current < cost { continue }
      if &dp[current - cost] == "" { continue }
      
      let next = (dp[current - cost].clone() + &num).to_string();
      if max(&next, &dp[current]) {
        dp[current] = next;
      }
    }
  }
  println!("{}", dp[n]);
}