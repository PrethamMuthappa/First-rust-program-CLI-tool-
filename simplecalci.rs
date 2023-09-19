use std::io::stdin as inp;
use std::io::stdout as out;
use std::io::Write;
fn main(){
loop {
		println!("1 add 2 sub 3 mul 4 div 5");
	let  mut choice:String=String::new();
    inp().read_line(&mut choice).unwrap();
    let choice:i32=match choice.trim().parse(){
    	Ok(num) =>num,
    	Err(_) =>{
    		println!("invalid input ");
    		continue;
    	}
    };

    if choice == 5 {
    	break;
    };

    let mut userinp1:String=String::new();
    let mut userinp2:String=String::new();
    println!("enter number 1");
    out().flush();
    inp().read_line(&mut userinp1).unwrap();
    println!("enter second number");
    out().flush();
    inp().read_line(&mut userinp2).unwrap();

    let num1:usize=match userinp1.trim().parse(){
    	Ok(num) => num,

    	Err(_) =>{
    		println!("error in firat number");
    		continue;
    	}
    };

    let num2:usize=match userinp2.trim().parse(){

    	Ok(num) => num,
    	Err(_) => {
    		println!("second number is not correct");
    		continue
    	}
    };
 
	match choice {
		1=> {println!("{:?}",add(num1,num2));},
		2=> {println!("{:?}",sub(num1,num2));},
		3=> {println!("{:?}",mul(num1,num2));},
		4=> {println!("{:?}",div(num1,num2));},
		_=> {
			eprintln!("invalid choice")
		}
	};
}

}

fn add(a:usize,b:usize)->usize{	
	return a+b;
}

fn sub(a:usize,b:usize)->usize{
	return a-b;
}

fn mul(a:usize,b:usize)->usize{
	return a*b;
}

fn div(a:usize,b:usize)->usize{
	return a/b;
}

