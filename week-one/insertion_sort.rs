// taken from https://rust.cc/article?id=71a9a469-fdc0-4752-894f-d654e0251c4a

pub fn insertion_sort(data: &mut [i32]) {
    // notice that len() won't consume the caller (even the call is a reference)
    for sorted in 0..data.len() {
        // fn min_by_key<B, F>(self, f: F) -> Option<Self::Item>
        // where -- B: Ord, F: FnMut(&Self::Item) -> B,
        let min = (sorted..data.len()).min_by_key(|&i| &data[i]).unwrap();
        // notice that " |&i| &data[i] " is a function of closure form
        data.swap(sorted, min);
    }
}

pub fn insertion_sort_fast_unsafe(data: &mut [i32]) {
    unsafe {
        for sorted in 0..data.len() {
            let min = (sorted..data.len())
                // unsafe function get_unchecked()
                // Returns a reference to an element or subslice, without doing bounds checking.
                .min_by_key(|&i| data.get_unchecked(i))
                .unwrap();
            std::ptr::swap(data.get_unchecked_mut(sorted), data.get_unchecked_mut(min));
        }
    }
}

// For above two functions
// two different kinds of "swap" function
// safe (slice::swap) -- pub fn swap(&mut self, a: usize, b: usize)
// Swaps two elements in the slice
// unsafe (std::ptr::swap) -- pub unsafe fn swap<T>(x: *mut T, y: *mut T)
// Swaps the values at two mutable locations of the same type, without deinitializing either


pub fn insertion_sort_fast_safe(data: &mut [i32]) {
    for sorted in 0..data.len() {
        let min = data
            .iter()
            // enumerate() : Creates an iterator which gives the current iteration count as well as the next value.
            .enumerate().skip(sorted)
            .min_by_key(|(i, e)| *e)
            .unwrap()
            .0;
        data.swap(sorted, min);
    }
}

pub fn insertion_sort_fast_iter(data: &mut [i32]) {
    for sorted in 0..data.len() {
        let min = data[sorted..].iter().enumerate().min_by_key(|&(_, e)| e).unwrap().0;
        data.swap(sorted, min + sorted);
    }
}