impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();
        let mut dif;

        for (i,n) in nums.iter().enumerate() {
            dif = target - n;
            if seen.contains_key(&dif) {
                return vec![*seen.get(&dif).unwrap(), i as i32];
            }
            seen.insert(n, i as i32);
        }
        vec![]
    }
}
