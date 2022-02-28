fn main()
{
    println!("Enter the size of side square :");
    let inputuser = get_input();
    for counter in 0..inputuser 
    {
        for counter1 in 0..inputuser 
        {
            print!("* ");   
        }
        print!("\n");
    }
}
fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}