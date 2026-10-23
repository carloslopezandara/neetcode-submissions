impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for token in tokens {
            match token.as_str() {
                "+" => {
                    let Some(second_op) = stack.pop() else { continue };
                    let Some(first_op) = stack.pop() else { continue };
                    let result = first_op + second_op;
                    stack.push(result);
                },
                "-" => {
                    let Some(second_op) = stack.pop() else { continue };
                    let Some(first_op) = stack.pop() else { continue };
                    let result = first_op - second_op;
                    stack.push(result);
                },
                "*" => {
                    let Some(second_op) = stack.pop() else { continue };
                    let Some(first_op) = stack.pop() else { continue };
                    let result = first_op * second_op;
                    stack.push(result);
                },
                "/" => {
                    let Some(second_op) = stack.pop() else { continue };
                    let Some(first_op) = stack.pop() else { continue };
                    let result = first_op / second_op;
                    stack.push(result);
                },
                _ => {
                    let current_n = token.parse::<i32>().unwrap();
                    stack.push(current_n);
                }
            }
        }
        stack.pop().unwrap()
    }
}
