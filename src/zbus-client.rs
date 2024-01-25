use std::time::Instant;
use tokio;
use zbus::{Connection, dbus_proxy, Result};

mod common;

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
	let (mode, count) = common::get_params();

	let conn = Connection::system().await?;
	let proxy = BenchmarkProxy::new(&conn).await?;

	let start = Instant::now();

	use common::BenchMode::*;
	match mode {
		Call => {
			for _ in 0..count {
				proxy.function_call().await.unwrap();
			}
		},
		Get => {
			for _ in 0..count {
				proxy.property().await.unwrap();
			}
		},
		Set => {
			for _ in 0..count {
				proxy.set_property(13).await.unwrap();
			}
		},
	};

	println!("{}", (Instant::now() - start).as_secs_f32());

	Ok(())
}
