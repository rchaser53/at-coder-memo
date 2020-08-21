use proconio::input;

fn main() {
  input! {
    n: usize,
    ais: [isize;n]
  }

  let mut sum = 0;
  for (i, v) in ais.iter().enumerate() {
    if i % 2 == 0 {
      sum += *v;
    } else {
      sum -= *v;
    }
  }
  
  let mut result = vec![0;n];
  result[0] = sum;
  let mut last = sum / 2;
  
  for i in 0..n-1 {
    last = ais[i] - last;
    result[i+1] = 2 * last;
  }
  
  let result: Vec<String> = result.into_iter().map(|v| v.to_string()).collect();
  println!("{}", result.join(" ")); 
}