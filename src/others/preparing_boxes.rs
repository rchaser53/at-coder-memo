use proconio::input;

fn main() {
  input! {
    n: usize,
    ais: [usize;n]
  }
  let mut result: Vec<usize> = vec![];
  let mut dp: Vec<usize> = vec![0;n+1];
  let mut index = n - 1;
  loop {
    let dp_index = index + 1;
    let mut sum = 0;
    
    for i in 1.. {
      let val = dp_index * i;
      if val > n {
        break
      }
      sum += dp[val];
    }
    
    if sum % 2 != ais[index] {
      result.push(dp_index);
      dp[dp_index] = 1;
    }
    
    if index == 0 {
      break
    }
    index -= 1;
  }
  
  result.sort();
  println!("{}", result.len());
  if !result.is_empty() {
    println!("{}", result
                    .into_iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
    );
  }
}