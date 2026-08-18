impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut i, mut j) = (0i32, numbers.len() as i32 -1);

        while i < j {
            if numbers[i as usize] + numbers[j as usize] == target {
                return vec![i+1,j+1];
            } else
            if numbers[i as usize] + numbers[j as usize] > target {
                j-=1;
            } else
            if numbers[i as usize] + numbers[j as usize] < target {
                i+=1;
            }
        }
        vec![]
    }
}
