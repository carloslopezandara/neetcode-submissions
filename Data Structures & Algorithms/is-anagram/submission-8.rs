impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut letters_count= HashMap::new();

        for letter in s.chars() {
            *letters_count.entry(letter).or_insert(0) +=1;
        }

        for letter in t.chars() {
            match letters_count.get_mut(&letter){
                Some(v) => {
                    *v -= 1;
                    if *v == 0 {
                        letters_count.remove(&letter);
                    }
                }
                None => return false
            }
        }

        letters_count.is_empty()
    }
}