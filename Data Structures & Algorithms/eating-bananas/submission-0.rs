impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        //eating speed = k = bananas/h
        //h >= piles.len()
        //min 1 max el mayor de piles

        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();
        let mut result = right;

        while left <= right {

            let mut k = left + (right - left) / 2;
            let mut time_needed = 0;

            for bananas in &piles {
                time_needed += (bananas+k-1)/k;
            }

            if time_needed <= h {
                result = k;
                right = k-1;
            }else {
                left = k+1;
            }
        }
        result
    }
}
