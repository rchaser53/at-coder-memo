use proconio::input;

fn main() {
  input! {
    n: usize,
    k: usize,
    mut h: [usize;n]
  }
  
  if n <= k {
    println!("0");
    return
  }
  
  h.sort();
  println!("{}", h.iter().by_ref().take(n-k).sum::<usize>());
}