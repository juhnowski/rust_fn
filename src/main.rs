fn main() {
    println!("Hello, world!");
    another_fn();
    arg_fn(1);
}

fn another_fn() {
    println!("Another fn");
}

fn arg_fn(x:i32) {
    println!("x={}", x);
}
