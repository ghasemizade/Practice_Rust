fn main ()
{
    // input
    println! ("Enter 5 number for operations :");
    let num1 = get_input ();
    let num2 = get_input ();
    let num3 = get_input ();
    let num4 = get_input ();
    let num5 = get_input ();
    
    let x: [i32; 5] = [num1, num2, num3, num4, num5];
    
    // difine variable
    let mut min = x [0];
    let mut max = x [0];
    let mut sum = 0;
    let mut avg = 0;
    let mut var = 0;
    let mut st = 0;
    let mut sum2 = 0;
    let mut a = 0;
    
    // the operation
    for counter in 0..5
    {
        if x [counter] < min
        {
            min = x [counter];
        }
        if x [counter] > max
        {
            max = x [counter];
        }
    }
    
    sum = x [0] + x [1] + x [2] + x [3] + x [4];
    avg = sum / 5;
    
    for counter1 in 0..5
    {
        st = x [counter1] - avg;
        sum2 = st * st;
        var = sum2 / 5;
    }
    
    for counter2 in 0..var
    {
        a = var / counter2;
        if a == counter2
        {
            break;
        }
    }
    
    // output opraition
    println! ("sum = {}", sum);
    println! ("avg = {}", avg);
    println! ("min = {}", min);
    println! ("max = {}", max);
    println!("varians = {}", var);
}

fn get_input() -> i32
{
    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}