use proconio::input;

const MOD: usize = 2019;
fn main() {
  input! {
    l: usize,
    r: usize
  }

  let diff = if r - l > MOD {
    2100
  } else {
    r - l
  };
  
  let mut stack = vec![];
  let mut min = usize::max_value();
  for i in l..=l+diff {
    stack.push(i);
  }
  
  for i in 0..stack.len() {
    for ii in i+1..stack.len() {
      min = std::cmp::min(min, stack[i] * stack[ii] % MOD); 
    }
  }

  println!("{}", min);
}