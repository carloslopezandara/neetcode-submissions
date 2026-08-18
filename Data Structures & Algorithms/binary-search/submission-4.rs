impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0usize;
        let mut finish = nums.len()-1;

        while start <= finish {
            let middle = start + (finish-start) / 2;
            if target > nums[middle] {
                start = middle+1;
            } else if target < nums[middle]  {
                if middle == 0 {
                    break;
                }
                finish = middle-1;
            } else {
                return middle as i32;
            }
        }
        -1
    }
}
