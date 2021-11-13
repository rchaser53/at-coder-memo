use proconio::input;

fn main() {
  input! {
    k: usize
  }  

  if k % 2 == 0 {
    println!("-1");
    return
  }
  
  let mut seven = 7;
  let mut count = 0;
  while count <= k {
    count += 1;
    seven %= k;
    if seven == 0 {
      println!("{}", count);
      return
    }
    
    seven = (seven * 10 + 7) % k;
  }
  
  println!("-1");
}