/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::*;

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
fn readchars() -> Vec<char> {
  let mut tmp = String::new();
  std::io::stdin().read_line(&mut tmp).ok();
  let tmp:String = tmp.trim().parse().ok().unwrap();
  tmp.chars().into_iter().collect::<Vec<char>>()
}

fn main() {
  let a:Vec<usize> = readvec();
  let (h, w) = (a[0],a[1]);
  let mut rows:Vec<Vec<usize>> = vec![];
  for _ in 0..h {
    rows.push(readvec());
  }

  let mut result = usize::max_value();
  for i in 0..h {
    for j in 0..w {
      let mut temp = 0;

      for k in 0..i {
        for l in 0..j {
          temp += rows[k][l] * std::cmp::min(i-k, j-l);
        }

        for l in j+1..w {
          temp += rows[k][l] * std::cmp::min(i-k, l-j);
        }
      }

      for k in i+1..h {
        for l in 0..j {
          temp += rows[k][l] * std::cmp::min(k-i, j-l);
        }

        for l in j+1..w {
          temp += rows[k][l] * std::cmp::min(k-i, l-j);
        }
      }
      result = std::cmp::min(result, temp);
    }
  }
  
  println!("{}", result);
}