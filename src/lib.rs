pub fn get_wheel_direction(x_vec:f32, y_vec:f32)->f32
{
    let mut vec_theta = (y_vec / x_vec).atan().to_degrees();
    if y_vec == 0.0
    {
        if x_vec > 0.0
        {
            vec_theta = 0.0;
        }
        else
        {
            vec_theta = 180.0;
        }
    }
    else if x_vec == 0.0
    {
        if y_vec > 0.0
        {
            vec_theta = 90.0;
        }
        else
        {
            vec_theta = -90.0;
        }
    }
    else
    {
        if x_vec < 0.0
        {
            vec_theta = vec_theta.abs() + 90.0;
        }
    
        if y_vec < 0.0
        {
            vec_theta = vec_theta.abs() + 90.0;
        }
    }

    if y_vec == 0.0 && x_vec == 0.0
    {
        vec_theta = 0.0;
    }

    if vec_theta > 180.0
    {
        vec_theta = vec_theta -180.0;
    }

    vec_theta.abs()
}

fn check_reversal(x_vec:f32, y_vec:f32)->bool
{
    // true = + false = -
    let mut reversal = true;
    let mut vec_theta = (y_vec / x_vec).atan().to_degrees();
    if y_vec == 0.0
    {
        if x_vec > 0.0
        {
            vec_theta = 0.0;
        }
        else
        {
            vec_theta = 180.0;
        }
    }
    else if x_vec == 0.0
    {
        if y_vec > 0.0
        {
            vec_theta = 90.0;
        }
        else
        {
            vec_theta = -90.0;
        }
    }

    if vec_theta < 0.0 || vec_theta > 180.0
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