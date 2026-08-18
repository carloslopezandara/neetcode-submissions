impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut longest = 0;

        let unique_nums: HashSet<i32> = nums.into_iter().collect();

        for n in &unique_nums {
            if !unique_nums.contains(&(n-1)) {
                let mut current_n = *n;
                let mut current_len=1;
                while unique_nums.contains(&(current_n+1)){
                    current_len +=1;
                    current_n +=1;
                }
                longest = longest.max(current_len);
            }        
        }
        longest
    }
}
