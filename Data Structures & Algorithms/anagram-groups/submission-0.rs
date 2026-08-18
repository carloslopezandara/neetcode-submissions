impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut words_map = HashMap::new();
        
        for s in strs{
            let mut current_string: Vec<char> = s.chars().collect();
            current_string.sort();
            
            words_map.entry(current_string).or_insert(vec![]).push(s);
        }
        
        words_map.into_values().collect()
    }
}
