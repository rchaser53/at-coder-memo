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

fn main() {
  let n:usize = readln();
  let mut points = vec![];
  for _ in 0..n {
    let a:Vec<f64> = readvec();
    points.push((a[0], a[1]));
  }
  
  let mut result = 0f64;
  let mut seen = vec![false;n];
  seen[0] = true;
  let mut now = (points[0].0, points[0].1);
  for _ in 1..n {
    let mut score = 1_000_000_000f64;
    let mut ti = 0;
    for j in 1..n {
      if seen[j] { continue }
      let nv = (now.0 - points[j].0).powi(2) + (now.1 - points[j].1).powi(2);
      if nv < score {
        score = nv;
        ti = j;
      }
    }
    seen[ti] = true;
    result += score.sqrt();
    now = points[ti];
  }

  result += ((points[0].0 - now.0).powi(2) + (points[0].1 - now.1).powi(2)).sqrt();
  println!("{}", result);
}