use proconio::input;
use proconio::marker::Usize1;
 
fn main() {
  input! {
    n: usize,
    k: isize,
    q: isize,
    resolvers: [Usize1;q as usize] 
  }
  let mut lifes = vec![k - q;n];
  for i in resolvers {
    lifes[i] += 1;
  }
  
  for v in lifes {
    if v > 0 {
      println!("Yes");
    } else {
      println!("No");
    }
  }
}