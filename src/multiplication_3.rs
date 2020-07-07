use proconio::input;
 
fn main() {
  input! {
      a: u64,
      b: String
  }

  let b:Vec<&str> = b.split(".").collect();
  let c = b[0].parse::<u64>().unwrap();
  let e = a * c;
  if b.len() == 1 {
    println!("{}", e);
  } else {
    let d = b[1].parse::<u64>().unwrap();
    let f = a * d / 100;
    println!("{}", e + f);
  }
}