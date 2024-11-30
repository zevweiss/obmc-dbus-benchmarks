#include <stdlib.h>
#include <stdio.h>

#include <systemd/sd-bus.h>

static int function_handler(sd_bus_message* msg, void* userdata, sd_bus_error* error)
{
	sd_bus_reply_method_return(msg, "");
	return 1;
}

static const sd_bus_vtable vtable[] = {
	SD_BUS_VTABLE_START(0),
	SD_BUS_WRITABLE_PROPERTY("Property", "i", NULL, NULL,
	                         0, SD_BUS_VTABLE_PROPERTY_EMITS_CHANGE),
	SD_BUS_METHOD("FunctionCall", "", "", function_handler, 0),
	SD_BUS_VTABLE_END
};

int main(int argc, char** argv)
{
	int propval = 0;
	sd_bus* bus;

	if (sd_bus_default_system(&bus) < 0) {
		fprintf(stderr, "sd_bus_default_system() failed\n");
		exit(1);
	}

	if (sd_bus_request_name(bus, "xyz.openbmc_project.Benchmark", 0) < 0) {
		fprintf(stderr, "dbus name request failed\n");
		exit(1);
	}

	if (sd_bus_add_object_vtable(bus, NULL,
	                             "/xyz/openbmc_project/benchmark",
	                             "xyz.openbmc_project.Benchmark",
	                             vtable, &propval) < 0) {
		fprintf(stderr, "sd_bus_add_object_vtable() failed\n");
		exit(1);
	}

	for (;;) {
		if (sd_bus_wait(bus, UINT64_MAX) < 0) {
			fprintf(stderr, "sd_bus_wait() failed\n");
			exit(1);
		}

		if (sd_bus_process(bus, NULL) < 0) {
			fprintf(stderr, "sd_bus_process() failed\n");
			exit(1);
		}
	}

	abort();
}
