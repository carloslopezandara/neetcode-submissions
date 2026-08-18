impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut unique_num = HashSet::new();

        for n in nums {
            if unique_num.contains(&n) {
                return true;
            }
            unique_num.insert(n);
        }
        false
    }
}
