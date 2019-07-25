use std::collections::HashMap;

macro_rules! new_map_trailing_comma {
	($($key: expr => $val:expr,)*) => {
		{
			let mut map = HashMap::new();

			$(
				map.insert($key, $val);
			)*

		map
		}
	};
}

macro_rules! new_map_no_trailing_comma {
	($($key: expr => $val:expr),*) => {
		{
			let mut map = HashMap::new();

			$(
				map.insert($key, $val);
			)*

		map
		}
	};
}

fn main() {
    let m0 = new_map_trailing_comma! {
        "one" => 1,
        "two" => 2,
        "three" => 3,
    };

    let m1 = new_map_no_trailing_comma! {
        "one" => 1,
        "two" => 2,
        "three" => 3
    };

    println!("{:?}", m0);
    println!("{:?}", m1);
}
