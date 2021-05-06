use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_map = HashMap::with_capacity(nums.len());

        for (i, &num) in nums.iter().enumerate() {
            let y = target - num;
            if let Some(&i2) = hash_map.get(&y) {
                return vec![i as i32, i2 as i32];
            } else {
                hash_map.insert(num, i);
            }
        }

        vec![]
    }
}