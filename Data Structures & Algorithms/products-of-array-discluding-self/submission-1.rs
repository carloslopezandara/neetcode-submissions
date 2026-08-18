impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut pre = 1;
        let mut post= 1;

        for n in &nums {
            result.push(pre);
            pre = pre * n;
        }

        for (i, n) in nums.into_iter().enumerate().rev(){
            result[i] *= post;
            post = post * n;
        }
        result
    }
}
