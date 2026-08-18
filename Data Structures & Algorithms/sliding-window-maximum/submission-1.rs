impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut left = 0;
    let mut queue = VecDeque::new();
    let mut result = Vec::new();
    if k<=1 {
        return nums;
    } else {

        for right in 0..nums.len() {
            while let Some(back) = queue.back(){
                if nums[right] > nums[*back]{
                    queue.pop_back();
                } else {
                    break;
                }
            }
            queue.push_back(right);

            while let Some(front) = queue.front() {
                if left > *front {
                    queue.pop_front();
                } else {
                    break;
                }
            }

            if (right - left + 1) >= k {
                result.push(nums[*queue.front().unwrap()]);
                left +=1;
            }                
        }
    }
    result
}
}
