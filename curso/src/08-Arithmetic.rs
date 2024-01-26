fn interproduct(a: u32, b: u32, c: u32) -> u32{
    return a * b + b * c + c * a; 
}

fn main(){
    println!("resultado: {}", interproduct(120, 100, 248))
}