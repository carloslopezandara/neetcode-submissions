impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut dif = 0;

        for (i,n) in nums.iter().enumerate() {
            dif = target-n;
            if map.contains_key(&dif) {
                return vec![*map.get(&dif).unwrap(),i as i32];
            }
            map.insert(n, i as i32);
        }
        vec![0,0]
    }
}
