// An Exercise in Matching 
//
// (a) if the bool is true and the int is between 20 and 26, 
// (b) if the bool is true and the aforementioned condition isn't true for the int, 
// (c) if the int is between 40 and 49 (where the value of the bool doesn't matter), and
// (d), wherein none of the previous conditions are true

fn main(){

    let x = (42 , true);
	
	match x{
	
		(20...26,true) => {
			println!("Case A");
			}
		(y,true) if y < 20 || y > 26  => {
			println!("Case B");
		}
		(40...49,_) => {
			println!("Case C");
		}
		(_,_) => {
			println!("Case D: None of conditions matched");
		}
	}

}


