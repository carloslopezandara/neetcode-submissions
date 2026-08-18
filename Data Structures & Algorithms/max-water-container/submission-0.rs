impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        //area = (j-i) * min(height[i],height[j])
        let (mut left_pointer, mut right_pointer) = (0, heights.len()-1);
        let mut result=0;

        while left_pointer < right_pointer {
            let current_area = (right_pointer as i32 - left_pointer as i32) * (heights[left_pointer].min(heights[right_pointer]));
            result = result.max(current_area);

            if heights[left_pointer] < heights[right_pointer] {
                left_pointer +=1;
            } else {
                right_pointer -=1;
            }
        }
        result
    }
}
