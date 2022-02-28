fn main()
{
    println!("Enter the number of number :");
    let inputuser = get_input();
    let mut sum = 0;
    for counter in 0..inputuser 
    {
        println!("Enter the {} number :", counter);
        let number = get_input();
        sum += number;
    }
    let mut average = 0;
    average = sum / inputuser;
    println!("average number is : {}", average);
}
fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}