use math;


#[test]
fn integrate_simple_tester()
{
	let res = integration::integrate_simple( &|x| x*x, 0.0, 3.0, 100);
	assert!( res < 9.1 && res > 8.9 );
	assert!( res >200.0);
}

