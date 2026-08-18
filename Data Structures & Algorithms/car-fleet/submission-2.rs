impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut pair: Vec<(i32,i32)> = position.into_iter().zip(speed.into_iter()).collect();
        pair.sort_by(|a, b| b.0.cmp(&a.0));
        let mut stack = Vec::new();

        for (position, speed) in pair {
            let current_time = (target - position) as f64 / speed as f64;
            stack.push(current_time);
            let len = stack.len();
            if len>=2 && stack[len-1] <= stack[len-2] {
                stack.pop();
            }
        }
        stack.len() as i32
    }
}









