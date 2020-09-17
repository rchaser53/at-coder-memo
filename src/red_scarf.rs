use proconio::input;


fn main() {
  input! {
    n: usize,
    mut vals: [usize;n],
  }
  	
  let mut base = 0;
  for i in 0..n {
    base ^= vals[i];
  }
  
  for i in 0..n {
    vals[i] ^= base;
  }
  
  println!("{}", vals
    .into_iter()
    .map(|v| v.to_string())
    .collect::<Vec<String>>()
    .join(" ")
  );
}