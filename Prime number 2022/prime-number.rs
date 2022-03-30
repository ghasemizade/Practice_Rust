fn main ()
{
  println! ("Enter a number for  :");
  let inputuser = get_input();
  let mut temp = 0;
  for counter in 1..inputuser
    {
      if inputuser%counter==0
      {
        temp = 1;
      }
    }
  if temp == 0
  {
    println!("yes");
  }
  else
  {
    println!("no");
  }
}

fn get_input() -> i32
{
    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}