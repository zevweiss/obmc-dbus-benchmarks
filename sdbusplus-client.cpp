#include <sdbusplus/bus.hpp>

enum benchmode
{
	call,
	get,
	set,
};

static const char* progname;

static void usage()
{
	fprintf(stderr, "Usage: %s [call|get|set] COUNT\n", progname);
	exit(1);
}

int main(int argc, char* argv[])
{
	sdbusplus::bus_t bus = sdbusplus::bus::new_system();
	enum benchmode mode;
	int count;

	progname = argv[0];

	if (argc != 3) {
		usage();
	}

	if (!strcmp(argv[1], "call"))
		mode = benchmode::call;
	else if (!strcmp(argv[1], "get"))
		mode = benchmode::get;
	else if (!strcmp(argv[1], "set"))
		mode = benchmode::set;
	else
		usage();

	count = atoi(argv[2]);

	auto start = std::chrono::steady_clock::now();

	switch (mode) {
	case benchmode::call:
		for (int i = 0; i < count; i++) {
			auto msg = bus.new_method_call("xyz.openbmc_project.Benchmark",
			                               "/xyz/openbmc_project/benchmark",
			                               "xyz.openbmc_project.Benchmark",
			                               "FunctionCall");
			auto resp = bus.call(msg);
		}
		break;

	case benchmode::get:
		for (int i = 0; i < count; i++) {
			std::variant<int> result;
			auto msg = bus.new_method_call("xyz.openbmc_project.Benchmark",
			                               "/xyz/openbmc_project/benchmark",
			                               "org.freedesktop.DBus.Properties",
			                               "Get");
			msg.append("xyz.openbmc_project.Benchmark", "Property");
			auto resp = bus.call(msg);
			resp.read(result);
			std::get<int>(result);
		}
		break;

	case benchmode::set:
		for (int i = 0; i < count; i++) {
			auto msg = bus.new_method_call("xyz.openbmc_project.Benchmark",
			                               "/xyz/openbmc_project/benchmark",
			                               "org.freedesktop.DBus.Properties",
			                               "Set");
			msg.append("xyz.openbmc_project.Benchmark", "Property", std::variant<int>(13));
			auto resp = bus.call(msg);
		}
		break;
	}

	auto duration = std::chrono::steady_clock::now() - start;
	auto duration_ms = std::chrono::duration_cast<std::chrono::milliseconds>(duration);
	long ms = duration_ms.count();
	printf("%ld.%03ld\n", ms / 1000, ms % 1000);

	return 0;
}
