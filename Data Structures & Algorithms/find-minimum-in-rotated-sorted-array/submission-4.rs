impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left=0usize;
        let mut right=nums.len()-1;
        let mut result = nums[0];

        while left <= right {
            if nums[left] < nums[right] {
                result = result.min(nums[left]);
                break;
            }

            let mid = left + (right - left) /2;
            result = result.min(nums[mid]);
            
            if  nums[mid] >= nums[left] {
                left = mid+1;
            } else {
                right= mid-1;
            }
        }
        result
    }
}


