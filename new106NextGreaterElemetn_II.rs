// The "weird iterator made for fun" translated to Rust
struct TwiceCircularIterator<'a> {
    nums: &'a [i32],
    n: usize,
    step: usize,
    max_steps: usize,
}

impl<'a> TwiceCircularIterator<'a> {
    fn new(nums: &'a [i32]) -> Self {
        let n = nums.len();
        Self {
            nums,
            n,
            step: 0,
            max_steps: 2 * n,
        }
    }

    fn has_next(&self) -> bool {
        self.step < self.max_steps
    }

    fn get_value(&self) -> i32 {
        self.nums[self.step % self.n]
    }

    fn get_real_index(&self) -> usize {
        self.step % self.n
    }

    fn is_first_pass(&self) -> bool {
        self.step < self.n
    }

    fn advance(&mut self) {
        self.step += 1;
    }
}

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![-1; n];
        
        let mut st: Vec<usize> = Vec::new(); 
        
        let mut it = TwiceCircularIterator::new(&nums);

        while it.has_next() {
            let val = it.get_value();
            let idx = it.get_real_index();

            while let Some(&top_idx) = st.last() {
                if nums[top_idx] < val {
                    ans[top_idx] = val;
                    st.pop();
                } else {
                    break;
                }
            }
            if it.is_first_pass() {
                st.push(idx);
            }   
            it.advance();
        }
        ans
    }
}
