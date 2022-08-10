fn readln<T: std::str::FromStr>() -> T {
    let mut tmp = String::new();
    std::io::stdin().read_line(&mut tmp).ok();
    tmp.trim().parse().ok().unwrap()
  }
  fn readvec<T: std::str::FromStr>() -> Vec<T> {
    readln::<String>()
        .split_whitespace()
        .map(|x| x.parse().ok().unwrap())
        .collect()
  }
  fn readchars() -> Vec<char> {
    let mut tmp = String::new();
    std::io::stdin().read_line(&mut tmp).ok();
    let tmp:String = tmp.trim().parse().ok().unwrap();
    tmp.chars().into_iter().collect::<Vec<char>>()
  }
  
  fn main() {
      let _:usize = readln();
      let a:Vec<usize> = readvec();
      let sum = a.iter().sum::<usize>();
  
      let mut result = 0;
      for v in a {
        result += v * sum;
      }
      println!("{}", result);
  }