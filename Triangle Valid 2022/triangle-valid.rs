fn main() {
    println! ("Enter a firstside :");
    let firstside = get_input ();
    println! ("Enter a secondside :");
    let secondside = get_input ();
    println! ("Enter a thirdside :");
    let thirdside = get_input ();
    
    //condition
    if firstside > secondside + thirdside
    {
        println! ("This is not a triangle.");
    }
    else
    {
        println! ("This is a triangle.");
    }
}
fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}
