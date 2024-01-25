use tokio;
use zbus::{ConnectionBuilder, dbus_interface, fdo, Result};

struct Benchmark {
	propval: i32,
}

#[dbus_interface(name = "xyz.openbmc_project.Benchmark")]
impl Benchmark {
	fn function_call(&mut self) -> fdo::Result<()> {
		Ok(())
	}

	#[dbus_interface(property)]
	fn property(&self) -> fdo::Result<i32> {
		Ok(self.propval)
	}

	#[dbus_interface(property)]
	fn set_property(&mut self, val: i32) -> fdo::Result<()> {
		self.propval = val;
		Ok(())
	}
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
	let bench = Benchmark { propval: 0 };
	let _conn = ConnectionBuilder::system()?
		.name("xyz.openbmc_project.Benchmark")?
		.serve_at("/xyz/openbmc_project/benchmark", bench)?
		.build()
		.await?;

	futures::future::pending::<()>().await;

	Ok(())
}
