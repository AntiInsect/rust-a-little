struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let (mut x, mut ret)  = (x as i64, 0);
        while x != 0 {
            // pick the number at a time
            ret = ret * 10 + x % 10;
            // update the origin number
            x = x / 10;
        }
        if ret > 2_i64.pow(31) - 1 || ret < -2_i64.pow(31) {
            return 0;
        }
        ret as i32
    }
}

fn main() {
    println!("the answer is {}", Solution::reverse(1534236469));
    println!("the answer is {}", Solution::reverse(0));
    println!("the answer is {}", Solution::reverse(201));
}
