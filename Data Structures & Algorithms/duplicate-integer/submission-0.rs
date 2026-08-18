impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut unique_nums = HashMap::new();
        for num in nums {
            if unique_nums.contains_key(&num) {
                return true;
            }
            unique_nums.insert(num.clone(),());
        }
        false
    }
}
