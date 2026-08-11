use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut Map : HashMap<i32,i32> = HashMap::new();
        let mut count = 0;
        let mut sum = 0;
        let n = nums.len();

        Map.insert(0,1);

        for i in 0..n{
            sum += nums[i];
            if let Some(&freq) = Map.get(&(sum-k)){
                count += freq;
            }
            *Map.entry(sum).or_insert(0) += 1;
        }
        count
    }
}
