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

    let node = ctx.create_node("twin_driver", None, Default::default())?;

    let subscriber = node.create_subscriber::<geometry_msgs::msg::Twist>("/cmd_vel", None)?;
    let left_power_publisher = node.create_publisher::<std_msgs::msg::Float32>("motor/power/left", None)?;
    let right_power_publisher = node.create_publisher::<std_msgs::msg::Float32>("motor/power/right", None)?;
    let direction_publisher = node.create_publisher::<std_msgs::msg::Float32>("motor/direction", None)?;

    let mut selector = ctx.create_selector()?;

    let mut history_direction = get_f64_parameter(node.get_name(), "init_pose", 90.0);

    let log = Logger::new(node.get_name());
    pr_info!(log, "Start:{}", node.get_name());

    selector.add_subscriber(
        subscriber, 
        Box::new(move |msg|{
            let x_vec = msg.linear.x as f32;
            let y_vec = msg.linear.y as f32;

            let target_theta = swerve_driver::get_wheel_direction(x_vec, y_vec);
            
        }),
    );

    loop {
        selector.wait()?;
    }
}