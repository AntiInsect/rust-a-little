use std::cmp::max;
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut i, mut j, mut max_length) = (0,0,0);
        let mut myhashset = HashSet::new();
        let chars = s.as_bytes();
        while j < chars.len() {
            // the char is already in the HashSet
            if !myhashset.insert(chars[j]) {
                max_length = max(max_length, j - i);
                while chars[i] != chars[j] {
                    myhashset.remove(&chars[i]);
                    i += 1;
                }
                // there can only be "ww" and cannot be "www" or more
                i += 1;
            }
            j += 1;
        }
        max(max_length, j - i) as i32
    }
}

fn main() {
    println!("the answer of {:?} is {}", "abcabcbb".to_string(), Solution::length_of_longest_substring("abcabcbb".to_string()));
    println!("the answer of {:?} is {}", "bbbbb".to_string(), Solution::length_of_longest_substring("bbbbb".to_string()));
    println!("the answer of {:?} is {}", "pwwkew".to_string(), Solution::length_of_longest_substring("pwwkew".to_string()));
}
