extern crate env_logger;
#[macro_use]
extern crate rosrust;
#[macro_use]
extern crate rosrust_codegen;

rosmsg_include!();

fn main() {
    env_logger::init();

    // Initialize node
    rosrust::init("talker");

    // Create publisher
    let mut chatter_pub_latched = rosrust::publish("chatter").unwrap();
    let mut chatter_pub_unlatched = rosrust::publish("chatter").unwrap();
    chatter_pub_latched.set_latching(true);

    let mut msg = msg::std_msgs::String::default();
    msg.data = String::from("hello world latched");

    // Log event
    ros_info!("Publishing: {}", msg.data);

    // Send string message to topic via publisher
    chatter_pub_latched.send(msg.clone()).unwrap();

    msg.data = String::from("hello world unlatched");

    // Log event
    ros_info!("Publishing: {}", msg.data);

    // Send string message to topic via publisher, without latching it
    chatter_pub_unlatched.send(msg).unwrap();

    rosrust::spin();
}
