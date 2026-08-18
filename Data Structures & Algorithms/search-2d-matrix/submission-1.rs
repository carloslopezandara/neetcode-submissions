impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        for vector in matrix {
            let mut start = 0usize;
            let mut finish = vector.len()-1;
            if target <= vector[finish] {
                while start <= finish {
                    let middle = start + (finish - start)/2;
                    if vector[middle] < target {                        
                        start = middle+1;
                    } else if vector[middle] > target {
                        if middle == 0 {
                            break;
                        }
                        finish = middle-1;
                    } else {
                        return true;
                    }
                }
            } else {
                continue;
            }
        }
        false
    }
}
