impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        
        for (i, num) in nums.iter().enumerate() {
            for (i2, num2) in nums.iter().enumerate() {
                if i == i2 {
                    continue;
                }
                
                if num + num2 == target {
                    res.push(i as i32);
                    res.push(i2 as i32);
                    
                    return res;
                }
            }
        }
        
        res
    }
}