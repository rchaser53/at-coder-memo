use proconio::input;
use proconio::marker::Usize1;

fn main() {
  input! {
    n: usize,
    nums: [Usize1;n]
  }
  
  let mut orders: Vec<(usize, usize)> = vec![];
  for (i, num) in nums.into_iter().enumerate() {
    orders.push((i, num));
  }
  
  orders.sort_by(|a, b| {
    a.1.partial_cmp(&b.1).unwrap()
  });
  
  let result = orders
  	.into_iter()
  	.map(|a| (a.0 + 1).to_string())
  	.collect::<Vec<String>>()
    .join(" ");
  
  println!("{}", result);
}