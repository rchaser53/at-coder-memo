use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    pairs: [(usize, String);m]
  }

  let mut arr: Vec<bool> = vec![false;n+1];
  let mut was_arr: Vec<usize> = vec![0;n+1];
  let mut was = 0;
  for (a, b) in pairs {
    match b.as_ref() {
      "AC" => arr[a] = true,
      "WA" => {
        if !arr[a] {
          was_arr[a] += 1
        }
      },
      _ => {}
    }
  }
  
  let mut acs = 0;
  for i in 1..=n {
    let v = arr[i];
    if v {
      acs += 1;
      was += was_arr[i];
    }
  }
  
  println!("{} {}", acs, was);
}
