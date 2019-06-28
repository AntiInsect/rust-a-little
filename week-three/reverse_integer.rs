struct Solution;


impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let (mut x, mut ret )= (x, 0);
        while x != 0 {
            // pick the number at a time
            ret = ret * 10 + x % 10;
            // update the origin number
            x = x / 10;
        }
        ret
    }
}

fn main() {
    println!("the answer is {}", Solution::reverse(1230));
    println!("the answer is {}", Solution::reverse(0));
    println!("the answer is {}", Solution::reverse(201));
}