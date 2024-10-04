
    fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut stack: Vec<i32> = Vec::new();
        
        for i in nums.iter(){
            if stack.is_empty() == true {
                stack.push(i.clone());
            } else {
                // compare to each on the stack
                for k in 1..=stack.len(){
                    if nums[i.clone() as usize] == stack[k]{
                        return true
                    }
                }
                stack.push(i.clone());
            }
            
            // how do I compare the first index in the vector to the stack
            // compare the value to the values on the stack
            
            // push the value to the stack
        }

        false
        
    }

    fn main(){

    }


