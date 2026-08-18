impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len(){
            return false;
        }

        let mut word_count = HashMap::new();

        for char in s.chars() {
            *word_count.entry(char).or_insert(0) +=1;
        }

        for char in t.chars() {
            match word_count.get_mut(&char) {
                Some(value) => {
                    *value -=1;
                    if *value == 0 {
                        word_count.remove(&char);
                    }
                }
                None => return false
            }
        }
        word_count.is_empty()
    }
}