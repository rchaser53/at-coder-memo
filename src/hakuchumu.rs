use std::collections::HashMap;
 
#[warn(non_snake_case)]
fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
  	s = s.trim().to_string();
  
    let mut cache: HashMap<&str, bool> = HashMap::new();
    
    if s.is_empty() {
      println!("NO");
      return
    }
  
    if helper(&s, &mut cache) {
      println!("YES");
    } else {
      println!("NO");
    }
}
 
const DICT: [(&str, usize); 4] = [
	("dream", 5), ("dreamer", 7), ("erase", 5), ("eraser", 6)
];
 
fn helper<'a>(input: &'a str, cache: &mut HashMap<&'a str, bool>) -> bool {
  	if let Some(flag) = cache.get(input) {
      return *flag
    }
  
  	if input.len() == 0 {
  		return true
    }
  
  	for (word, size) in DICT.iter() {
    	if input.starts_with(word) && helper(&input[*size..], cache) {
			return true
      	}
    }
  
  	cache.insert(input, false);
	false
}