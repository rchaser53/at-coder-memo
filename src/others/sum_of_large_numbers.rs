use proconio::input;

fn sum(l: usize, r:usize)-> usize {
  (l + r) * (r-l+1) / 2
}

fn main() {
  let Mod = 1_000_000_007;
  input! {
    N: usize,
    K: usize
  }
  
  let mut result = 0;
  
  for i in K..=N+1 {
    let l = sum(0, i-1);
    let r = sum(N-i+1, N);
    result = (result + r - l + 1) % Mod;
  }
  
  println!("{}", result);
}
