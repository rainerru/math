mod integration;

use integration::*;

fn main()
{
	println!("Let's do some integration:");
	println!("The integral of f(x)=x^2 on [0,3] is {}", integrate_simple( &|x| x*x, 0.0, 3.0, 100));
	println!("The integral of g(x)=x on [0,3] is {}", integrate_simple( &|x| x, 0.0, 3.0, 100));
	println!("Now with Simpson's rule:");
	println!("The integral of f(x)=x^2 on [0,3] is {}", integrate_simpson_n( &|x| x*x, 0.0, 3.0, 100));
	println!("The integral of g(x)=x on [0,3] is {}", integrate_simpson_n( &|x| x, 0.0, 3.0, 100)); 
	println!("Now with Simpson's rule, withouth specifying n:");
	println!("The integral of f(x)=x^2 on [0,3] is {}", integrate_simpson( &|x| x*x, 0.0, 3.0));
	println!("The integral of g(x)=x on [0,3] is {}", integrate_simpson( &|x| x, 0.0, 3.0)); 

}


