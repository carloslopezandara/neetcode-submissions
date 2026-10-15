impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams = HashMap::new();

        for s in strs {
            let mut current_string: Vec<char> = s.chars().collect();
            current_string.sort();
            anagrams.entry(current_string).or_insert(vec![]).push(s);
        }

        anagrams.into_values().collect()

    }
}
