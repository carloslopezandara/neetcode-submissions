impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix_product: Vec<i32> = Vec::new();
        let mut postfix_product: Vec<i32> = Vec::new();
        let mut result: Vec<i32> = Vec::new();
        let mut pre = 1;
        let mut post = 1;

        let mut nums_reverse = nums.clone();
        nums_reverse.reverse();
        let mut i=0;

        while i < nums.len() {
            if i==0 {
                prefix_product.push(1);
                postfix_product.push(1);
            } else {
                prefix_product.push(pre);
                postfix_product.push(post);
            }
            pre *= nums[i];
            post *= nums_reverse[i];

            i+=1;
        }

        postfix_product.reverse();

        i=0;
        while i < nums.len() {
            result.push(prefix_product[i]*postfix_product[i]);
            i+=1;
        }

        result
    }
}
