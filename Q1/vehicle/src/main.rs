//Q1  changes Sollution

mod Vehicle
{
  pub mod Corolla
{
 pub fn start()
{
println!("Start the car");
}
}
}
use crate::Vehicle::Corolla;

fn main()
 {   
    Corolla::start();
}

