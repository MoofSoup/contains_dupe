
struct HanoiIterator{
    state: Vec<(usize, usize, usize, usize)>,

}impl HanoiIterator {
    fn new(n: usize) -> Self {
        HanoiIterator {
            state: vec![(n, 0, 1, 2)],
        }
    }
}

impl Iterator for HanoiIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((n, from, aux, to)) = self.state.pop() {
            if n == 1 {
                return Some((from, to));
            } else {
                self.state.push((n - 1, aux, from, to));
                self.state.push((1, from, aux, to));
                self.state.push((n - 1, from, to, aux));
               
            }
        }
        None
    }
}
struct CompAllIterator {
    stack: Vec<(usize,usize, usize)>,
}
impl CompAllIterator {
    fn new(n: usize)-> CompAllIterator {
        CompAllIterator {
            stack: vec! {(n, n, n - 1)}
        }
    }
}
impl Iterator for CompAllIterator {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        // if the base case n == 1
        while let Some((n, first_operand, second_operand)) = self.stack.pop() {
            if n == 1 {
                return Some((first_operand, second_operand));
            } else {
                // make a recursive call
                // return the value
                self.stack.push((n - 1, first_operand , second_operand - 1));
                self.stack.push((n - 1, first_operand - 1, second_operand - 1));
                self.stack.push((1, first_operand, second_operand));
                
            
            }
        }
        None

        // return Some((0, 1));

    }



}
fn contains_duplicate(nums: Vec<i32>) -> bool {
        for (left_index, right_index) in CompAllIterator::new(nums.len()-1){
            if nums[left_index] == nums[right_index] {
                return true
            }
        }
        false
}

    fn main(){
       let w_dupes = vec!{1, 2, 3};
       let l_dupes = vec!{1, 2, 3, 3};
        let comp = CompAllIterator::new(w_dupes.len());
        for (left_operand, right_operand) in comp{
            println!("hello {left_operand} {right_operand}" )
        }
        let a = contains_duplicate(w_dupes);
        let b = contains_duplicate(l_dupes);
        println!("a is {a}, b is {b}");
    }


