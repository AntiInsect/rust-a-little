// When taking a closure as an "input parameter"
// the closure's complete type must be annotated using one of a few traits
// Fn: the closure captures by reference (&T)
// FnMut: the closure captures by mutable reference (&mut T)
// FnOnce: the closure captures by value (T)

fn call_fn(x: i32, f: Fn) where
f: Fn(i32) -> i32
{
    println!("{}", f(x));
}

fn test_fn()
{
    let j = 5;
    let g = |i: i32| i + j;
    call_fn(2, g);
}

fn call_fnmut(x: i32, mut f: impl FnMut(i32) -> ())
{
    f(x);
}

fn test_fnmut()
{
    let mut j = 5;
    let g = |i: i32| j = i + j;
    call_fnmut(2, g);
    println!("{}", &j);
//    error : the environment var changed
//    call_fn(2, g);
}

fn call_fnonce<F:FnOnce()>(f:F){
    f()
}

fn tset_fnonce() {
    let mut v = vec![];
    let g = | | {
        v.push(1);
        println!("v:{:?}",v);
        v
    };
    call(g);
//    g();
}