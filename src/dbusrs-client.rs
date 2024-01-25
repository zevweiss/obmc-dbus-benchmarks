use std::time::Duration;
use dbus::nonblock::stdintf::org_freedesktop_dbus::Properties;
use dbus_tokio::connection;

type ErrResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main(flavor = "current_thread")]
async fn main() -> ErrResult<()> {
	let (bus_resource, bus) = connection::new_system_sync()?;
	let _handle = tokio::spawn(async {
		let err = bus_resource.await;
		panic!("Lost connection to D-Bus: {}", err);
	});

	let proxy = dbus::nonblock::Proxy::new("xyz.openbmc_project.Benchmark",
					       "/xyz/openbmc_project/benchmark",
					       Duration::from_secs(2), bus);

	let t: i32 = proxy.get("xyz.openbmc_project.Benchmark", "Property").await?;
	println!("Property: {}", t);
	proxy.set("xyz.openbmc_project.Benchmark", "Property", 13).await?;
	let t: i32 = proxy.get("xyz.openbmc_project.Benchmark", "Property").await?;
	println!("Property: {}", t);
	proxy.method_call("xyz.openbmc_project.Benchmark", "FunctionCall", ()).await?;

	Ok(())
}
