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

fn helper(vals: &Vec<usize>, i:usize, j:usize) -> bool {
	if i == 0 {
		return j == 0
	}
	
    let mut flag = false;
	if j >= vals[i-1] && helper(vals, i-1, j-vals[i-1]) {
		flag = true;
	}
    if helper(vals, i-1, j) {
		flag = true;
	}
	flag
}

fn main() {
	let vals: Vec<usize> = readvec();
	let (n,x) = (vals[0], vals[1]);
	let vals: Vec<usize> = readvec();

	if helper(&vals, n, x) {
		println!("Yes");
	} else {
		println!("No");
	}	
}
