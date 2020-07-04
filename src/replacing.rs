use proconio::input;
use std::collections::HashMap;

struct Info {
	count: u64,
  	val: u64,
}

fn main() {
    input! {
        n: usize,
      	arr: [u64;n],
      	q: usize
    }
  	
  	let mut sum = 0;
  	let mut map: HashMap<u64, Info> = HashMap::new();
  	for i in 0..n {
    	let mut info = map.entry(arr[i]).or_insert(Info {
        	count: 0,
          	val: arr[i]
        });
      	sum += arr[i];
        info.count += 1;
    }
  	
  	for _ in 0..q {
      input! {
          from: u64,
          to: u64
      }
      if let Some(mut from_info) = map.remove(&from) {
        sum -= (from - to) * from_info.count;
        
        from_info.val = to;
        if let Some(mut to_info) = map.get_mut(&to) {
            to_info.count += from_info.count;
        } else {
            map.insert(to, from_info);
        }
      }
      println!("{}", sum);
    }
}