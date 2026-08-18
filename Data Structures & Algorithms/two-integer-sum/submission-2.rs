impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_watched = HashMap::new();
        let mut dif;
        for (i,n) in nums.iter().enumerate() {
            dif = target - n;
            if nums_watched.contains_key(&dif) {
                return vec![*nums_watched.get(&dif).unwrap(), i as i32];
            }
            nums_watched.insert(n, i as i32);
        }
        vec![]
    }
}
