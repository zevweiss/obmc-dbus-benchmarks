use tokio;
use zbus::{Connection, dbus_proxy, Result};

#[dbus_proxy(interface = "xyz.openbmc_project.Benchmark",
	     default_service = "xyz.openbmc_project.Benchmark",
	     default_path = "/xyz/openbmc_project/benchmark")]
trait Benchmark {
	#[dbus_proxy(property)]
	fn property(&self) -> Result<i32>;

	#[dbus_proxy(property)]
	fn set_property(&self, value: i32) -> Result<()>;

	fn function_call(&self) -> Result<()>;
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
	let conn = Connection::system().await?;
	let proxy = BenchmarkProxy::new(&conn).await?;

	println!("Property: {}", proxy.property().await?);
	proxy.set_property(12).await?;
	println!("Property: {}", proxy.property().await?);
	proxy.function_call().await?;

	Ok(())
}
