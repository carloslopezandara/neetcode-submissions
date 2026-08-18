impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let (mut left_pointer, mut right_pointer) = (0i32, s.len() as i32 -1);

        let word = s.as_bytes();

        while left_pointer < right_pointer {
            while left_pointer < right_pointer && !word[left_pointer as usize].is_ascii_alphanumeric() {
                left_pointer+=1;
            }
            while right_pointer > left_pointer && !word[right_pointer as usize].is_ascii_alphanumeric() {
                right_pointer-=1;
            }

            if word[left_pointer as usize].to_ascii_lowercase() != word[right_pointer as usize].to_ascii_lowercase() {
                return false;
            }
            left_pointer +=1;
            right_pointer -=1;
        }
        true
    }
}
