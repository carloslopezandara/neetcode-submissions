impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let lenght = s.len();
        if lenght != t.len(){
            return false;
        }        
        let mut letters_in_s = HashMap::new();
        let mut letters_in_t = HashMap::new();
        let chars_in_s:Vec<char> = s.chars().collect();
        let chars_in_t:Vec<char> = t.chars().collect();

        let mut i = 0;
        while i < lenght {
            letters_in_s.entry(chars_in_s[i]).and_modify(|count| {*count +=1}).or_insert(1);
            letters_in_t.entry(chars_in_t[i]).and_modify(|count| {*count +=1}).or_insert(1);
            i+=1;
        }

        letters_in_s == letters_in_t
    }
}