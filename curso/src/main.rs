fn fib(n: u32) -> u32{
    let ret: u32;
    if n <= 2{
        ret = 1;
    }else {
        ret = n + (n/2);
    }

    return ret;
}

fn main() {
    let n: u32 = 25;
    println!("fib(n) = {}", fib(n));
}
