//let x = (24 , true);


fn main(){

    let x = (24 , true);
	
	match x{
	
		(20...26,true) => {
			println!("Case A");
			}
		(_,true) => {
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


