impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len()!=t.len() {
            return false;
        }

        let letters_in_s: Vec<char> = s.chars().collect();
        let letters_in_t: Vec<char> = t.chars().collect();
          
        let mut chars_in_s = HashMap::new();
        let mut chars_in_t = HashMap::new();

        let mut i=0;
        while i < s.len(){
            *chars_in_s.entry(letters_in_s[i]).or_insert(0)+=1;
            *chars_in_t.entry(letters_in_t[i]).or_insert(0)+=1;
            i+=1;
        }
        
        chars_in_s==chars_in_t
        
    }
}