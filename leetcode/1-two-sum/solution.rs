impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut complements: Vec<i32> = Vec::new();
        for (i, num) in nums.iter().enumerate() {
            match complements.iter().position(|n| n == num) {
                Some(item) => return vec![item as i32, i as i32],
                None => complements.push(target - num),
            };
        };
        vec![]
    }
}
