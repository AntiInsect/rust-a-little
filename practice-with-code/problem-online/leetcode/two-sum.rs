use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut myhash:HashMap<i32, i32> = HashMap::new();

        for (index, &num) in nums.iter().enumerate() {
            let target_to_check = target - num;
            if myhash.contains_key(&target_to_check) {
                return vec![*myhash.get(&target_to_check).unwrap(), index as i32];
            }
            myhash.insert(num,index as i32);
        }
        return vec![];
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    println!("the result is {:?}", Solution::two_sum(nums, target));
}