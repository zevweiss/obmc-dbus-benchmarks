#include <stdlib.h>
#include <stdio.h>

#include <systemd/sd-bus.h>

enum benchmode
{
	BM_CALL,
	BM_GET,
	BM_SET,
};

static const char* progname;

static void usage()
{
	fprintf(stderr, "Usage: %s [call|get|set] COUNT\n", progname);
	exit(1);
}

int main(int argc, char* argv[])
{
	sd_bus* bus;
	enum benchmode mode;
	int count;
	struct timespec start_ts, end_ts;
	double start, end;

	progname = argv[0];

	if (argc != 3) {
		usage();
	}

	if (!strcmp(argv[1], "call"))
		mode = BM_CALL;
	else if (!strcmp(argv[1], "get"))
		mode = BM_GET;
	else if (!strcmp(argv[1], "set"))
		mode = BM_SET;
	else
		usage();

	count = atoi(argv[2]);

	if (sd_bus_default_system(&bus) < 0) {
		fprintf(stderr, "sd_bus_default_system() failed\n");
		exit(1);
	}

	if (clock_gettime(CLOCK_MONOTONIC, &start_ts)) {
		perror("clock_gettime");
		exit(1);
	}
	start = (double)start_ts.tv_sec + ((double)start_ts.tv_nsec / 1000000000.0);

	switch (mode) {
	case BM_CALL:
		for (int i = 0; i < count; i++) {
			sd_bus_error error = SD_BUS_ERROR_NULL;
			sd_bus_message* reply = NULL;
			if (sd_bus_call_method(bus,
			                       "xyz.openbmc_project.Benchmark",
			                       "/xyz/openbmc_project/benchmark",
			                       "xyz.openbmc_project.Benchmark",
			                       "FunctionCall", &error, &reply, "") < 0) {
				fprintf(stderr, "sd_bus_call_method() failed\n");
				exit(1);
			}
			sd_bus_message_unrefp(&reply);
			sd_bus_error_free(&error);
		}
		break;

	case BM_GET:
		for (int i = 0; i < count; i++) {
			sd_bus_error error = SD_BUS_ERROR_NULL;
			int propval;
			if (sd_bus_get_property_trivial(bus,
			                                "xyz.openbmc_project.Benchmark",
			                                "/xyz/openbmc_project/benchmark",
			                                "xyz.openbmc_project.Benchmark",
			                                "Property", &error, 'i', &propval) < 0) {
				fprintf(stderr, "sd_bus_get_property_trivial() failed\n");
				exit(1);
			}
			sd_bus_error_free(&error);
		}
		break;

	case BM_SET:
		for (int i = 0; i < count; i++) {
			sd_bus_error error = SD_BUS_ERROR_NULL;
			if (sd_bus_set_property(bus,
			                        "xyz.openbmc_project.Benchmark",
			                        "/xyz/openbmc_project/benchmark",
			                        "xyz.openbmc_project.Benchmark",
			                        "Property", &error, "i", 13) < 0) {
				fprintf(stderr, "sd_bus_set_property() failed\n");
				exit(1);
			}
			sd_bus_error_free(&error);
		}
		break;
	}

	if (clock_gettime(CLOCK_MONOTONIC, &end_ts)) {
		perror("clock_gettime");
		exit(1);
	}
	end = (double)end_ts.tv_sec + ((double)end_ts.tv_nsec / 1000000000.0);

	printf("%.3f\n", end - start);

	return 0;
}
