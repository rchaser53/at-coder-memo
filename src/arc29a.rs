fn main() {
  proconio::input!{
    n: usize,
    mut vals: [usize;n]
  }
    
  for _ in n..4 {
    vals.push(0);
  }
  
  let vs = vec![
    std::cmp::max(vals[0]+vals[1], vals[2]+vals[3]),
    std::cmp::max(vals[0]+vals[2], vals[1]+vals[3]),
    std::cmp::max(vals[0]+vals[3], vals[1]+vals[2]),
    std::cmp::max(vals[0], vals[1]+vals[2]+vals[3]),
    std::cmp::max(vals[1], vals[0]+vals[2]+vals[3]),
    std::cmp::max(vals[2], vals[1]+vals[0]+vals[3]),
    std::cmp::max(vals[3], vals[1]+vals[2]+vals[0]),
  ];
  let mut min = 10000;
  for v in vs {
    min = std::cmp::min(min, v);
  }
  println!("{}", min);
}