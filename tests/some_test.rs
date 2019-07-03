use math;
use math::integration;


#[test]
fn integrate_simple_tester()
{
	let res = integration::integrate_simple( &|x| x*x, 0.0, 3.0, 100);
	assert!( res < 9.5 && res > 8.5,
		"integrate_simple of x^2 from 0 to 3 should be 9, but was {}",
		res
	);
}

#[test]
fn multiple_integration_tester()
{
	let pool: Vec<fn(&Fn(f64) -> f64 , f64, f64, u32 ) -> f64>
		= vec![integration::integrate_simple, integration::integrate_simpson_n];
	let description = vec![ "integrate_simple", "integrate_simpson_n" ];
	let mut i = 0;
	
	for f in pool
	{
		let res = f( &|x| x*x, 0.0, 3.0, 100);
		assert!( res < 9.5 && res > 8.5,
			"integration of {} of x^2 from 0 to 3 should be 9, but was {}",
			description[i], res
		);
		i = i+1;
	}
}


