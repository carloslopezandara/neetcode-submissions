impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();

        for (i,n) in nums.iter().enumerate() {
            let dif = target-n;
            if let Some(j) = seen.get(&dif) {
                return vec![*j,i as i32];
            } else {
                seen.insert(n, i as i32);
            }
        }

        vec![]
    }
}
