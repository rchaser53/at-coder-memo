use proconio::input;
 
fn main() {
  input! {
    s: String,
  }
  
  let chars: Vec<char> = s.chars().collect();
  let mut result: Vec<String> = vec![String::from("0");chars.len()];  
  let mut index = 0;
  while index < chars.len() {
    let mut r_count = 0;
    let mut l_count = 0;
    while chars[index] == 'R' {
      index += 1;
      r_count += 1;
    }
    let r_index = index - 1;
    while let Some(char) = chars.get(index) {
      if char != &'L' {
        break
      }
      index += 1;
      l_count += 1;
    }
    
    let r_val = if r_count % 2 == 1 {
      r_count / 2 + 1
    } else {
      r_count / 2
    };
    
    let l_val = if l_count % 2 == 1 {
      l_count / 2 + 1
    } else {
      l_count / 2
    };

    result[r_index] = (r_val + l_count - l_val).to_string();
    result[r_index + 1] = (r_count - r_val + l_val).to_string();
  }
  println!("{}", result.join(" "));
}