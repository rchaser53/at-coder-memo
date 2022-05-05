use std::collections::*;

struct Leaderboard {
  btreemap: BTreeMap<i32,HashSet<i32>>,
  removed: HashMap<i32,i32>,
  dict: HashMap<i32, i32>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Leaderboard {
    fn new() -> Self {
        Leaderboard {
          btreemap: BTreeMap::new(),
          removed: HashMap::new(),
          dict: HashMap::new()
        }
    }
    
    fn add_score(&mut self, player_id: i32, score: i32) {
      let score = if let Some(val) = self.dict.get_mut(&player_id) {
        if let Some(map) = self.btreemap.get_mut(&val) {
          map.remove(&player_id);
        }
        *val += score;
        *val
      } else {
        self.dict.insert(player_id, score);
        score
      };

      let entry = self.btreemap.entry(score).or_insert(HashSet::new());
      entry.insert(player_id);
    }
    
    fn top(&self, mut k: i32) -> i32 {
      let mut temp = 0;
      for (v, set) in self.btreemap.iter().rev() {
        let len = set.len() as i32;
        if k <= len {
          temp += v * k;
          return temp
        }

        k -= len;
        temp += v * len;
      }
      unreachable!()
    }
    
    fn reset(&mut self, player_id: i32) {
      let val = self.dict.get(&player_id).unwrap();
      if let Some(map) = self.btreemap.get_mut(val) {
        map.remove(&player_id);
      }
      self.dict.remove(&player_id);
    }
}