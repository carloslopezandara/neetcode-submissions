impl Solution {
    pub fn is_valid(s: String) -> bool {
    if s.is_empty() {
        return true;
    }
    if s.len() % 2 != 0 {
        return false;
    }
    //opening meto a stack / closing saco y comparo usar rust match para comparacion exahustiva
    let s_bytes = s.as_bytes();
    let mut stack = Vec::new();
    let parentheses:HashMap<u8,u8>=[(b')', b'('), (b']', b'['), (b'}', b'{')].into();

    for &char in s_bytes {
        if char == b'{' || char == b'(' || char == b'[' {
            stack.push(char);
        } else {
            if let Some(e) = stack.pop() {
                if e != parentheses[&char] {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    stack.is_empty()
    }   
}
