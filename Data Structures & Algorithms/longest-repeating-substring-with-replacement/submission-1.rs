impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        // mientras pueda moverme right avanza, si la ventana falla aumenta left para acortarla
        // eso se gestiona con la condicion especifica en este caso que no exceda k
        let mut count = HashMap::new();
        let s = s.as_bytes();
        let k = k as usize;
        let mut left = 0;
        let mut max_f= 0;
        let mut res = 0;

        for right in 0..s.len() {
            let current = count.entry(s[right]).or_insert(0);
            *current +=1;
            max_f = max_f.max(*current);

            while (right - left + 1) - max_f > k {
                let current = count.get_mut(&s[left]).unwrap();
                *current -= 1;
                left += 1;
            }
            res = res.max(right - left + 1);
        }

        res as i32
    }
}






















