use safe_drive::{
    context::Context,
    error::DynError,
    logger::Logger,
    msg::common_interfaces::{geometry_msgs, std_msgs},
    pr_info
};

use ros2_rust_util::get_f64_parameter;
use swerve_driver;

fn main()->Result<(), DynError>
{
    let ctx = Context::new()?;

    let node = ctx.create_node("limit_rotation_driver", None, Default::default())?;

    let subscriber = node.create_subscriber::<geometry_msgs::msg::Twist>("/cmd_vel", None)?;
    let left_power_publisher = node.create_publisher::<std_msgs::msg::Float32>("motor/power/left", None)?;
    let right_power_publisher = node.create_publisher::<std_msgs::msg::Float32>("motor/power/right", None)?;
    // degree
    let direction_publisher = node.create_publisher::<std_msgs::msg::Float32>("motor/direction", None)?;

    let mut selector = ctx.create_selector()?;

    let mut history_direction = get_f64_parameter(node.get_name(), "init_wheel_direction", 90.0) as f32;

    let log = Logger::new(node.get_name());
    pr_info!(log, "Start:{}", node.get_name());

    selector.add_subscriber(
        subscriber, 
        Box::new(move |msg|{
            // message
            let mut l_pow_msg = std_msgs::msg::Float32::new().unwrap();
            let mut r_pow_msg = std_msgs::msg::Float32::new().unwrap();
            let mut direction_msg = std_msgs::msg::Float32::new().unwrap();

            // Get Vector
            let x_vec = msg.linear.x as f32;
            let y_vec = msg.linear.y as f32;
            let rotation_vec = msg.angular.z as f32;

            // Calc
            let target_vec = swerve_driver::get_free_vec_power(x_vec, y_vec);
            let target_direction = swerve_driver::get_free_wheel_direction(x_vec, y_vec);
            
            // send message
            l_pow_msg.data = target_vec * 0.5 + rotation_vec * 0.5;
            r_pow_msg.data = target_vec * 0.5 - rotation_vec * 0.5;
            direction_msg.data = target_direction - history_direction;

            let _ = left_power_publisher.send(&l_pow_msg);
            let _ = right_power_publisher.send(&r_pow_msg);
            let _ = direction_publisher.send(&direction_msg);

            history_direction = target_direction;
        }),
    );

    loop {
        selector.wait()?;
    }
}