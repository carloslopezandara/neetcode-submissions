impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut nums_seen = HashSet::new();
        for num in nums {
            if nums_seen.contains(&num) {
                return true;
            }
            nums_seen.insert(num);
        }
        return false;
    }
}
