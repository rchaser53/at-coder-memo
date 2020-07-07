use proconio::input;

fn main() {
  input! {
      n: usize,
      mut arr: [u64; n]
  }

  arr.sort();
  if arr.len() == 2 {
    println!("{}", arr[1]);
    return
  }

  if arr.len() == 3 {
    println!("{}", arr[1] + arr[2]);
    return
  }

  let mut result = arr.pop().unwrap();
  let mut base_stack: Vec<u64> = vec![arr.pop().unwrap()];
  while (0 <= arr.len()) {
    let mut stack: Vec<u64> = vec![];
    while 0 < base_stack.len() {
      let val = base_stack.remove(0);
      if arr.len() == 0 {
        println!("{}", result);
        return
      }
      stack.push(arr.pop().unwrap());
      result += val;

      if arr.len() == 0 {
        println!("{}", result);
        return
      }
      stack.push(arr.pop().unwrap());
      result += val;
    }
    base_stack = stack;
  }

  println!("{}", result);
}