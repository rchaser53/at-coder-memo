use proconio::input;
 
fn main() {
  input! {
    n: usize,
    m: usize,
    mut drinks: [(usize, usize);n]
  }
  drinks.sort_by(|a, b| a.0.cmp(&b.0));
  
  let mut total = 0;
  let mut count = 0;
  for (val, num) in drinks {
    for _ in 0..num {
      count += 1;
      total += val;
      
      if m == count {
        println!("{}", total);
        return
      }
    }  
  }
}