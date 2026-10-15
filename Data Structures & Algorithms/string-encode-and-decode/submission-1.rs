impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut encoded_string = String::new();
        
        for s in strs {
            let len = s.len().to_string();
            encoded_string.push_str((len+"#"+&s).as_str());
        }
        encoded_string
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut decoded = Vec::new();
        let mut encoded_string = s.as_bytes();
        let mut i=0;

        while i < s.len() {
            let mut j=i;
            while encoded_string [j] != b'#' {
                j+=1;
            }

            let len: usize = s[i..j].parse().unwrap();

            j+=1;

            decoded.push(s[j..j+len].to_string());

            i=j+len;
        }
        decoded
    }
}
