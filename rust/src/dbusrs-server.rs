use dbus_tokio::connection;
use dbus_crossroads::Crossroads;

type ErrResult<T> = Result<T, Box<dyn std::error::Error>>;

struct Benchmark {
	propval: i32,
}

fn register_iface_cb(b: &mut dbus_crossroads::IfaceBuilder<Benchmark>) {
	b.method("FunctionCall", (), (), |_, _, _: ()| {
		Ok(())
	});

	b.property("Property")
		.get(|_, b: &mut Benchmark| Ok(b.propval))
		.set(|_, b: &mut Benchmark, v| {
			b.propval = v;
			Ok(Some(v))
		});
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ErrResult<()> {
	let (bus_resource, bus) = connection::new_system_sync()?;
	let _handle = tokio::spawn(async {
		let err = bus_resource.await;
		panic!("Lost connection to D-Bus: {}", err);
	});

	let mut cr = Crossroads::new();
	cr.set_async_support(Some((bus.clone(), Box::new(|x| { tokio::spawn(x); }))));
	cr.set_object_manager_support(Some(bus.clone()));

	let iface = cr.register("xyz.openbmc_project.Benchmark", register_iface_cb);

	cr.insert("/xyz", &[], ());
	cr.insert("/xyz/openbmc_project", &[], ());
	cr.insert("/xyz/openbmc_project/benchmark", &[iface], Benchmark { propval: 0 });

	use dbus::channel::MatchingReceiver;
	bus.start_receive(dbus::message::MatchRule::new_method_call(), Box::new(move |msg, conn| {
		cr.handle_message(msg, conn).expect("Crossroads::handle_message() failed");
		true
	}));

	let reply = bus.request_name("xyz.openbmc_project.Benchmark",
				     false, false, true).await?;
	if reply != dbus::nonblock::stdintf::org_freedesktop_dbus::RequestNameReply::PrimaryOwner {
		panic!("failed to acquire dbus name");
	}

	futures::future::pending::<()>().await;

	Ok(())
}
