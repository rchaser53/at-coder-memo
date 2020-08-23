use proconio::input;

fn main() {
  input! {
    n: usize,
    k: usize,
    ais: [usize;n]
  }
  
  let mut total = 0;
  let mut sum = ais[0];
  let mut left = 0;
  let mut right = 0;
  while right < ais.len() {
    if k <= sum {
      total += ais.len() - right;
      sum -= ais[left];
      left +=1;
      if right < left {
        right += 1;
        if right >= ais.len() {
          break;
        }
        sum += ais[right];
      }
    } else {
      right += 1;
      if right >= ais.len() {
        break;
      }
      sum += ais[right];
    }
  }
  println!("{}", total);
}