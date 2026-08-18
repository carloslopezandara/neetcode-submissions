impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut result = vec![0i32;temperatures.len()];

        for (i,&n) in temperatures.iter().enumerate() {
            while let Some(&back) = stack.last() {
                if temperatures[back] < n {
                    let index = stack.pop().unwrap();
                    result[index]=(i - index) as i32;
                }else {
                    break
                }
            } 
            stack.push(i);
        }
        result
    }
}
