// iter() -> &T
// iter() -> &mut T
// into_iter() -> T
// iterator is a more basic concept than pointer


// fold function
fn use_names_for_something_else(_names: Vec<&str>) {}
fn test_fold() {
    let names = vec!["Jane", "Jill", "Jack", "John"];

    let total_bytes = names
        .iter()
        .map(|name: &&str| name.len()) // this line can be changed
        						       // .map(|&&name| name.len()) -- the closure captures the variable
        .fold(0, |acc, len| acc + len ); // running sum standard usage

    assert_eq!(total_bytes, 16);
    use_names_for_something_else(names);
}

// sort_by funciton
fn test_sort_by() {
    let mut teams = [
        [ ("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19), ],
        [ ("Bill", 17), ("Brenda", 16), ("Brad", 18), ("Barbara", 17), ]
    ];

    let teams_in_score_order = teams
        .iter_mut()
        .map(|team| {
            team.sort_by(|&a, &b| a.1.cmp(&b.1).reverse()); // like cmp funciton passed to c++ qsort
            team
        })
        .collect::<Vec<_>>();
    println!("Teams: {:?}", teams_in_score_order);
}



// the iter & loop
fn error_loop_0() {
    let values = vec![1, 2, 3, 4];
	// ownship moved
    for x in values {
        println!("{}", x);
    }

    let y = values; // move error
}

fn error_loop_1 {
	let values = vec![1, 2, 3, 4];
	// make no sense that we consume the vec itself
	let mut it = values.into_iter();
	loop {
	    match it.next() {
	        Some(x) => println!("{}", x),
	        None => break,
	    }
	}
}

fn test_loop() {
    let values = vec![1, 2, 3, 4];
    for x in &values {
        println!("{}", x);
    }
    let y = values; // perfectly valid
}


// core::iter::Cloned
fn test_iter() {
    let x = vec!["Jill", "Jack", "Jane", "John"];

    let iter_all = x
        .clone() // clone the vector
        .into_iter() // into_iter() will consume the caller
        .collect::<Vec<_>>();


    let iter_first_two_0 = x
        .clone()
        .into_iter()
        .take(2) // take() is usually used to transfer infinite iter to finite one
        		 // also see skip()
        .collect::<Vec<_>>();

    let iter_first_two_1 = x
        .iter()
        .map(|i| i.clone())
        .take(2)
        .collect::<Vec<_>>();

    let iter_first_two_2 = x
        .iter()
        .cloned() // clone the iter (watch the type)
        .take(2)
        .collect::<Vec<_>>();
}

