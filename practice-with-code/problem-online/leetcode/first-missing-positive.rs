struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let len = nums.len();
        let mut i = 0;
        while i < len {
            let num = nums[i];
            // segregate the negative and positive
            // we don't care those value larger than the (len + 1)
            if num > 0 && num - 1 < (len as i32) {
                // swap: https://doc.rust-lang.org/std/primitive.slice.html#method.swap
                nums.swap((num - 1) as usize, i);
                if (num - 1) > (i as i32) && (num != nums[i]) {
                    continue;
                }
            }
            i += 1;
        }
        for (k, &num) in nums.iter().enumerate() {
            if num != ((k + 1) as i32) {
                return (k + 1) as i32
            }
        }
        return (len + 1) as i32
    }
}


