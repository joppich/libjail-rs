extern crate jail;

use std::process::Command;

use jail::param;
use jail::process::Jailed;

fn main() {
    let mut stopped = jail::StoppedJail::new("/rescue")
        .name("example_basic")
        .ip("127.0.1.1".parse().expect("couldn't parse IP Addr"))
        .ip("fe80::2".parse().expect("couldn't parse IP Addr"))
        .param("kern.osreldate",param::Value::Int(1525749716))
        .param("kern.osrelease",param::Value::String("11.1-RELEASE-p10".to_string()))
        .param("allow.raw_sockets", param::Value::Int(1))
        .param("allow.sysvipc", param::Value::Int(1));

    stopped.hostname = Some("testjail.example.org".to_string());

    let running = stopped.start().expect("Failed to start jail");

    println!("created new jail with JID {}", running.jid);

    println!(
        "the jail's path is {:?}",
        running.path().expect("could not get path")
    );

    println!(
        "the jail's jailname is '{}'",
        running.name().expect("could not get name")
    );

    println!(
        "the jail's IP addresses are: {:?}",
        running.ips().expect("could not get ip addresses")
    );

    println!("Other parameters: {:#?}", running.params().unwrap());

    println!("Let's run a command in the jail!");
    let output = Command::new("/hostname")
        .jail(&running)
        .output()
        .expect("Failed to execute command in jail");

    println!("output: {}", String::from_utf8_lossy(&output.stdout));

    println!("jid before restart: {}", running.jid);
    let running = running.restart().unwrap();
    println!("jid after restart: {}", running.jid);

    running.kill().expect("Failed to stop Jail");
}
