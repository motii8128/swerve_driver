use swerve_driver::get_wheel_direction;

fn main()
{

    let b = get_wheel_direction(0.0, -1.0);

    println!("{}", b);
}