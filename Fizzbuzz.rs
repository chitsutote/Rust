// Author: Wei-Cheng,Ling
// Date : 01/29/2014
//
// Description: 
//             	Printing the numbers from 1 to 100 
//             	But for multiples of three print "Fizz"
//             	multiples of five print "Buzz"
//            	multiples of both three and five print "FizzBuzz"


fn main(){

    //1u32 means nubmer 1 in type unsigned integers 1 u32
	for i in 1u32..101{
		
		if i % 15 == 0 {
			println!("FizzBuzz");
		}else if i % 3 == 0{
			println!("Fizz");
		}else if i % 5 == 0{
			println!("Buzz");
		}else{
			println!("{}",i);
		}
	
	}

}
