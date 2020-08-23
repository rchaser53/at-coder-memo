use proconio::input;

fn main() {
  input! {
    n: usize,
    mut tasks: [(usize, usize);n]
  }
  tasks.sort_by(|a, b| a.1.cmp(&b.1));

  if tasks[0].0 > tasks[0].1 {
    println!("No");
    return
  }
  
  let mut last_time = tasks[0].0;
  for i in 1..tasks.len() {
    let (take_time, dead_line) = tasks[i];
    
    if last_time + take_time > dead_line {
      println!("No");
      return
    }
    last_time += take_time;
  }
  
  if last_time > tasks[tasks.len()-1].1 {
    println!("No");
  } else {
    println!("Yes");
  }
}