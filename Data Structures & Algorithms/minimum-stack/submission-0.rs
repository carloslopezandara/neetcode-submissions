struct MinStack {
    stack : Vec<i32>,
    min_value: Vec<i32>
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_value: Vec::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        let min_val = if let Some(&current_min_value) = self.min_value.last() {
            current_min_value.min(val)
        } else {
            val
        };
        self.min_value.push(min_val);
        self.stack.push(val);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
        self.min_value.pop();
    }

    pub fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        *self.min_value.last().unwrap()
    }
}