impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut encoded_string = String::new();
        let mut len: String;
        for s in strs {
            len = s.len().to_string();
            encoded_string.push_str((len+"#"+&s).as_str());
        }
        encoded_string
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut decoded_string = Vec::new();
        let bytes = s.as_bytes();
        let mut i=0;

        while i < s.len() {
            let mut j=i;
            while bytes [j] != b'#' {
                j +=1 ;
            }

            let len: usize = s[i..j].parse().unwrap();

            j += 1;

            let word = s[j..j+len].to_string();

            decoded_string.push(word);

            i=j+len;

        }

        decoded_string
    }
}
