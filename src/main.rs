
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
        println!("function next() called!");
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
    stack: Vec<(usize, usize, usize, usize)>,
}
impl CompAllIterator {
    fn new(n: usize)-> CompAllIterator {
        if n == 0 {
            CompAllIterator {
                stack: vec! {(0, 0, 0, 0)}
            }
        } else {
        let n = n - 1;
        CompAllIterator {
            stack: vec! {(1, n - 1, n, n)}
        }
        }
    }
}
impl Iterator for CompAllIterator {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        // if there is 1 comparison to be made
        
        while let Some((n, l, r, rmax)) = self.stack.pop() {
            if rmax == 0{
                return None;
                
            }
            if n == 0 {

                return Some((l, r));
            } else {
                // make a recursive call
                // return the value
                // in the base case l == 0 && r == rmax
                if (l == 0 && r == rmax){
                    self.stack.push((0, l, r, rmax))
                }
                else if r == rmax {
                    let l = l - 1;
                    let r = l + 1;
                    self.stack.push((1, l, r, rmax));
                } else {
                    let r = r + 1;
                    self.stack.push((1, l, r, rmax));
                }
                if !(l == 0 && r == rmax) {
                    self.stack.push((0, l, r, rmax));
                }

                
                
                
            
            }
        }
        
        None

        // return Some((0, 1));

    }



}
fn contains_duplicate(nums: Vec<i32>) -> bool {
        for (left_index, right_index) in CompAllIterator::new(nums.len()){
            if nums[left_index] == nums[right_index] {
                return true
            }
        }
        false
}

    fn main(){
       let w_dupes = vec!{0, 1, 2, 3};
       println!("there is a duplicate: {}", contains_duplicate(w_dupes));
       let l_dupes = vec!{1, 2, 3, 3};
        let comp = CompAllIterator::new(l_dupes.len());
        // for (left_operand, right_operand) in comp{
        //     println!("hello {left_operand} {right_operand}" )
        // }
        // let a = contains_duplicate(w_dupes);
        // let b = contains_duplicate(l_dupes);
        // println!("a is {a}, b is {b}");
        println!("the length is {}", l_dupes.len());
        for (step, (l_index, r_index)) in CompAllIterator::new(l_dupes.len()).enumerate() {
            println!("{step}: {l_index}, {r_index}");
        }
    }


