impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut pre=1;
        let mut post=1;
        let mut result = Vec::new();

        for n in &nums {
            result.push(pre);
            pre*=n;
        }

        for (i,n) in nums.into_iter().enumerate().rev() {
            result[i] *=post;
            post*=n;
        }
    result
    }
}
