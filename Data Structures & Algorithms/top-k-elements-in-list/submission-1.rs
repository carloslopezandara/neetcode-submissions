impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut numbers = HashMap::new();
        let mut freq = vec![vec![]; nums.len() + 1];

        for n in nums {
            *numbers.entry(n).or_insert(0) +=1;
        }

        for (i, n) in numbers {
            freq[n].push(i);
        }

        let mut res = Vec::new();
        for i in (1..freq.len()).rev() {
            for num in &freq[i] {
                res.push(*num);
                if res.len() == k as usize {
                    return res;
                }
            }
        }
        res
    }
}
