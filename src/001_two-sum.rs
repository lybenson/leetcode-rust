use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mp = HashMap::new();
        for i in 0..nums.len() {
            let it = mp.get(&(target - nums[i]));
            match it {
                Some(val) => {
                    return vec![i as i32, *val];
                },
                None => mp.insert(nums[i], i as i32),
            };
        }
        Vec::new()
    }
}