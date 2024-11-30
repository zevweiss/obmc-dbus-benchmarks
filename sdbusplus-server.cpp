#include <sdbusplus/asio/object_server.hpp>

int main(int argc, char* argv[])
{
	boost::asio::io_context io;
	std::shared_ptr<sdbusplus::asio::connection> conn;
	std::shared_ptr<sdbusplus::asio::dbus_interface> iface;

	conn = std::make_shared<sdbusplus::asio::connection>(io);
	conn->request_name("xyz.openbmc_project.Benchmark");
	sdbusplus::asio::object_server server = sdbusplus::asio::object_server(conn);
	iface = server.add_interface("/xyz/openbmc_project/benchmark",
	                             "xyz.openbmc_project.Benchmark");
	iface->register_property("Property", 0, [](const int& req, int& newval) {
		newval = req;
		return 1;
	});
	iface->register_method("FunctionCall", [] {});
	iface->initialize();

	io.run();

	return 0;
}
