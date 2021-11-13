use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    mut ms: [isize;m]
  }
  
  if m <= n {
    println!("0");
    return
  }
    
  ms.sort();
  if n == 1 {
    println!("{}", (ms.last().unwrap() - ms.first().unwrap()).abs());
    return
  }
  
  let mut between: Vec<isize> = vec![0;m-1];
  for i in 0..m-1 {
    let v = ms[i];
    between[i] = (ms[i+1] - ms[i]).abs();
  }
  between.sort();
  
  for _ in 0..n-1 {
    between.pop();
  }
  
  println!("{}", between.iter().sum::<isize>());
}