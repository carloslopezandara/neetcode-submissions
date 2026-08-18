impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut seen = HashMap::new();
        let s = s.as_bytes();
        let mut result = 0;
        let mut left = 0usize;

        for rigth in 0..s.len() {
            if let Some(index) = seen.get(&s[rigth]) {
                left = left.max(index+1);
            }
            seen.insert(s[rigth], rigth);
            result = result.max(rigth-left+1);
        }
        result as i32
    }
}
