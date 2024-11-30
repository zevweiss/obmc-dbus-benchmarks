use std::env::args;

pub enum BenchMode {
	Call,
	Set,
	Get,
}

fn usage() -> ! {
	eprintln!("Usage: {} {{call,set,get}} [COUNT]",
	          args().next().unwrap_or("[progname]".into()));
	std::process::exit(1);
}

pub fn get_params() -> (BenchMode, usize) {
	let mode = match args().nth(1).unwrap_or_else(|| usage()).as_str() {
		"call" => BenchMode::Call,
		"set" => BenchMode::Set,
		"get" => BenchMode::Get,
		_ => usage(),
	};

	let count: usize = args().nth(2)
		.unwrap_or("100".into())
		.parse()
		.unwrap_or_else(|_| usage());

	(mode, count)
}
