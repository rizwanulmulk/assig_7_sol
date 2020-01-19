
mod pakistan;
//using crate
use crate::pakistan::seasons::four_seasons;
fn main()
 {   
    println!("call the library in the same folder");
    //using direct calling the mod
   // pakistan::seasons::four_seasons::winter();

  //OR using crate   
 four_seasons::winter();
}

