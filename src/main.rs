fn main() {
    let a: Vec<i32> = vec![];
    
    println!("a size: {}", a.len());

    let mut b = a;
    b.push(5);
    
    b.push(3);
    println!("b size: {}", b.len());
  }