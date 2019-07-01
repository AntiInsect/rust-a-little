//this function does not need to be made public
fn isprime(n : u32) -> bool {
    !(2..n).any(|num| n % num == 0)
}

pub fn nth(n :u32) -> u32 {
    (2..).filter(|num| isprime(*num)).skip(n as usize).next().unwrap()
}


