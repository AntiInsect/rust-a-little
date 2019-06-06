use std::fmt;

struct CompoundTime {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    min: usize,
    sec: usize,
}

macro_rules! reduce {
    ($s: ident, $(($from: ident, $to: ident, $factor: expr)),+) => {{
        $(
            $s.$to += $s.$from / $factor;
            $s.$from %= $factor;
        )+
    }}
}

impl CompoundTime {
    fn new(year: usize, month: usize, day: usize, hour: usize, min: usize, sec: usize) -> Self {
        CompoundTime {
            year,
            month,
            day,
            hour,
            min,
            sec,
        }
    }

    fn rearrange(&mut self) {
        reduce!(
            self,
            (sec, min, 60),
            (min, hour, 60),
            (hour, day, 24),
            (day, month, 30),
            (month, year, 12)
        );
    }
}

impl fmt::Display for CompoundTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}year {}month {}day {}hour {}min {}sec",
            self.year, self.month, self.day, self.hour, self.min, self.sec
        )
    }
}

fn main() {
    let mut ct = CompoundTime::new(0, 0, 21, 2341, 1231, 13);
    println!("Before: {}", ct);
    ct.rearrange();
    println!("After: {}", ct);
}
