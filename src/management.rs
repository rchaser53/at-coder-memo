use proconio::input;
 
fn main() {
  input! {
    N: usize,
    A: [usize;N-1],
  }
 
  let mut result: Vec<usize> = Vec::with_capacity(N+1);
  for _ in 0..=N {
    result.push(0);
  }
 
  for i in A.into_iter() {
    result[i] += 1;
  }
  
  for i in 1..=N {
    println!("{}", result[i]);
  }
}