use async_std::channel::unbounded;
use safe_drive::{context::Context, error::DynError, logger::Logger, pr_info, msg::common_interfaces::std_msgs};

use rust_robo_utils;

#[async_std::main]
async fn main()->Result<(), DynError>
{
    let context = Context::new()?;

    let node = context.create_node("reciever", None, Default::default())?;

    let publisher = node.create_publisher::<std_msgs::msg::Float32>("test", None)?;

    let logger = Logger::new("turtle_test");

    let addr = "127.0.0.1:8080";

    let (sender, reciever) = unbounded();

    let reciever_task = async_std::task::spawn(rust_robo_utils::connector::udp_reciever(addr, reciever, publisher));

    let signal_task = async_std::task::spawn(rust_robo_utils::connector::get_signal(sender));

    reciever_task.await?;
    signal_task.await?;

    pr_info!(logger, "shutdown reciever");

    Ok(())
}
