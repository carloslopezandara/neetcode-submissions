impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0usize;
        let mut right = nums.len()-1;
        let mut result=-1;

        while left <= right {
            let mid = left + (right - left)/2;
            if nums[mid] == target{
                result=mid as i32;
                break;
            } else if nums[left] <= nums [mid] {                
                if nums[left] <= target && target < nums[mid]{
                    right=mid-1;
                } else {
                    left=mid+1;
                }
            } else {
                if nums[mid] < target && target <= nums[right]{
                    left=mid+1;
                } else {
                    right=mid-1;
                }
            }
        }
        result
    }
}
//[3,4,5,6,1,2]
//t=1
//l=4, r=5 / 1 2
//mid = 4 / 1


