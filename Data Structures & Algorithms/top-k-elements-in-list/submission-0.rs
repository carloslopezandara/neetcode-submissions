impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut frequency = HashMap::new();
        
        for n in nums {
            *frequency.entry(n).or_insert(0)+=1;
        }

        let mut heap = BinaryHeap::new();

        for (num, freq) in frequency {
            heap.push((freq, num));
        }

        (0..k)
            .filter_map(|_| heap.pop().map(|(_,num)| num)).collect()
        
    }
}
