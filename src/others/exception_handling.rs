use proconio::input;
 
fn main() {
  input! {
    n: usize,
    ais: [usize;n]
  }

  let mut copied = ais.clone();
  copied.sort();
  let max = copied[n-1];
  let second_max = copied[n-2];
  
  for v in ais {
    if max == v {
      println!("{}", second_max);
    } else {
      println!("{}", max);
    }
  }
}