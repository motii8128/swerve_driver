pub fn get_wheel_direction(x_vec:f32, y_vec:f32)->f32
{
    let quadrant;
    let mut vec_theta = (y_vec.abs() / x_vec.abs()).atan().to_degrees();

    if y_vec == 0.0 && x_vec == 0.0
    {
        vec_theta = 0.0;
    }

    if x_vec > 0.0
    {
        if y_vec > 0.0
        {
            quadrant = 1;
        }
        else
        {
            quadrant = 4;
        }
    }
    else
    {
        if y_vec > 0.0
        {
            quadrant = 2;
        }
        else
        {
            quadrant = 3;
        }
    }

    if quadrant == 2
    {
        vec_theta += 90.0;
    }
    else if quadrant == 4
    {
        vec_theta += 90.0;
    }

    


    vec_theta
}

pub fn get_free_wheel_direction(x_vec:f32, y_vec:f32)->f32
{
    let quadrant;
    let mut vec_theta = (y_vec.abs() / x_vec.abs()).atan().to_degrees();

    if y_vec == 0.0 && x_vec == 0.0
    {
        vec_theta = 0.0;
    }

    if x_vec > 0.0
    {
        if y_vec > 0.0
        {
            quadrant = 1;
        }
        else
        {
            quadrant = 4;
        }
    }
    else
    {
        if y_vec > 0.0
        {
            quadrant = 2;
        }
        else
        {
            quadrant = 3;
        }
    }

    if quadrant == 2
    {
        vec_theta += 90.0;
    }
    else if quadrant == 3
    {
        vec_theta += 180.0;
    }
    else if quadrant == 4
    {
        vec_theta += 270.0;
    }

    


    vec_theta
}

fn check_reversal(x_vec:f32, y_vec:f32)->bool
{
    // true = + false = -
    let mut reversal = true;
    
    let quadrant;

    if x_vec > 0.0
    {
        if y_vec > 0.0
        {
            quadrant = 1;
        }
        else
        {
            quadrant = 4;
        }
    }
    else
    {
        if y_vec > 0.0
        {
            quadrant = 2;
        }
        else
        {
            quadrant = 3;
        }
    }

    if quadrant > 2
    {
        reversal = false;
    }

    reversal
}


pub fn get_vec_power(x_vec:f32, y_vec:f32)->f32
{
    let mut target_vec = (x_vec.powi(2) + y_vec.powi(2)).sqrt();

    if !check_reversal(x_vec, y_vec)
    {
        target_vec = target_vec * -1.0;
    }
    
    target_vec
}

pub fn get_free_vec_power(x_vec:f32, y_vec:f32)->f32
{
    let target_vec = (x_vec.powi(2) + y_vec.powi(2)).sqrt();

    target_vec
}