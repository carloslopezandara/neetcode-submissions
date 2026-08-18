impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        // nos vamos a mover siempre hacia el lado mas grande tanto por izq como der,
        // calculamos maximo hacia donde me muevo ejem: max izq - h[i] sabiendo que 
        // siempre en la otra direccion hay un muro grande mas grande que te permite 
        // abarcar el agua.
        
        if height.is_empty() {
            return 0;
        }

        let (mut left_pointer, mut right_pointer) = (0, height.len()-1);
        let (mut max_left, mut max_right) = (height[left_pointer],height[right_pointer]);
        let mut result=0;

        while left_pointer < right_pointer {
            if max_left<max_right {
                left_pointer +=1;
                max_left=max_left.max(height[left_pointer]);
                result += max_left - height[left_pointer];
            } else {
                right_pointer -=1;
                max_right=max_right.max(height[right_pointer]);
                result += max_right - height[right_pointer];
            }
        }
        result
    }
}
