use std::time::{Duration, Instant};
use dbus::nonblock::stdintf::org_freedesktop_dbus::Properties;
use dbus_tokio::connection;

mod common;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let (mode, count) = common::get_params();

	let (bus_resource, bus) = connection::new_system_sync()?;
	let _handle = tokio::spawn(async {
		let err = bus_resource.await;
		panic!("Lost connection to D-Bus: {}", err);
	});

	let proxy = dbus::nonblock::Proxy::new("xyz.openbmc_project.Benchmark",
					       "/xyz/openbmc_project/benchmark",
	                                       Duration::from_secs(2), bus);

	let start = Instant::now();

	use common::BenchMode::*;
	match mode {
		Call => {
			for _ in 0..count {
				let _: () = proxy.method_call("xyz.openbmc_project.Benchmark", "FunctionCall", ()).await.unwrap();
			}
		},
		Get => {
			for _ in 0..count {
				let _: i32 = proxy.get("xyz.openbmc_project.Benchmark", "Property").await.unwrap();
			}
		},
		Set => {
			for _ in 0..count {
				proxy.set("xyz.openbmc_project.Benchmark", "Property", 13).await.unwrap();
			}
		},
	};

	println!("{}", (Instant::now() - start).as_secs_f32());

	Ok(())
}
