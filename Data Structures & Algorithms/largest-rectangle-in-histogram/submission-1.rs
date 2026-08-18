impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        //si abarca lo meto si es menor hago pop calculo y ya veo si lo meto o no

        let mut stack: Vec<(usize, i32)> = Vec::new();
        let mut largest_area = 0;

        for (i,&h) in heights.iter().enumerate() {
            let mut start = i;
            while let Some(&(idx, height)) = stack.last(){
                if h < height {
                    stack.pop();
                    let current_area =(i-idx) as i32 * height;
                    largest_area=largest_area.max(current_area);
                    start = idx;      
                } else {
                    break;
                }
            }
            stack.push((start, h));
        }        
        let len = heights.len();
        //println!("{:?}, {:?}", stack, len);
        for (i, h) in stack.iter().rev() {
            let current_area =(len-i) as i32 * h;
            largest_area=largest_area.max(current_area);
        }
        largest_area
    }
}
