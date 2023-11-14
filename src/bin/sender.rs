use safe_drive::{context::Context, error::DynError, logger::Logger, pr_info, msg::common_interfaces::std_msgs};
use async_std::channel::unbounded;
use rust_robo_utils;

#[async_std::main]
async fn main()->Result<(), DynError>
{
    let context = Context::new()?;

    let node = context.create_node("sender", None, Default::default())?;

    let subscriber = node.create_subscriber::<std_msgs::msg::Float32>("test_topic", None)?;

    let logger = Logger::new("turtle_test");

    let sender_addr = "127.0.0.1:34543";
    let reciever_addr = "127.0.0.1:8080";

    let (sender, reciever) = unbounded();

    let sender_task = async_std::task::spawn(rust_robo_utils::connector::udp_sender(sender_addr, reciever_addr, reciever, subscriber));
    let signal_task = async_std::task::spawn(rust_robo_utils::connector::get_signal(sender));

    sender_task.await?;
    signal_task.await?;

    pr_info!(logger, "shutdown sender");

    Ok(())
}
