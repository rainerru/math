

pub fn integrate_simple ( f: &Fn(f64) -> f64 , a: f64, b: f64, n: u32 ) -> f64
{
    let mut integral    : f64 = 0.0_f64;
    let mut pos         : f64 = a;
    let increment       : f64 = (1.0/n as f64) * b-a;

    for _index in 0..n
    {
        integral += f( pos );
        pos += increment;
    }

    return integral*(b-a)/(n as f64) as f64;
}


pub fn integrate_simpson ( f: &Fn(f64) -> f64 , a: f64, b: f64 ) -> f64
{
	let mut n : u32 = 64;
	let mut oldintegral : f64 = 1.0;
	let mut newintegral : f64 = 0.0;

	while (oldintegral - newintegral).abs() >= 0.00001
	{
		oldintegral = newintegral;
		newintegral = integrate_simpson_n( &f, a, b, n );
		n *= n;
	}

	newintegral
}


pub fn integrate_simpson_n ( f: &Fn(f64) -> f64 , a: f64, b: f64, n: u32 ) -> f64
{
    let mut integral    : f64 = f(a);
    let mut pos         : f64 = a;
    let increment       : f64 = (0.5/n as f64) * b-a;

    for _index in 0..n
    {
        pos += increment;
        integral += 4.0*f( pos );
        pos += increment;
        integral += 2.0*f( pos );
    }

    integral -= f(pos);

    integral *= (b-a)/(6.0*n as f64);

    return integral;
}

