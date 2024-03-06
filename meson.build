project(
    'c-dbus-bench',
    'c',
    'cpp',
    default_options: [
        'warning_level=3',
        'cpp_std=c++23'
    ],
    license: 'Apache-2.0',
    version: '0.1',
    meson_version: '>=1.1.1',
)

add_project_arguments(
    '-Wno-psabi',
    language: 'cpp',
)

cpp_args = []

deps = [
  dependency('sdbusplus'),
]

executable(
  'dbusbench-sdbusplus-server',
  'sdbusplus-server.cpp',
  cpp_args: cpp_args,
  dependencies: deps,
  install: true,
  install_dir: get_option('bindir'))

executable(
  'dbusbench-sdbusplus-client',
  'sdbusplus-client.cpp',
  cpp_args: cpp_args,
  dependencies: deps,
  install: true,
  install_dir: get_option('bindir'))

executable(
  'dbusbench-sdbus-server',
  'sdbus-server.c',
  c_args: [],
  dependencies: dependency('libsystemd'),
  install: true,
  install_dir: get_option('bindir'))

executable(
  'dbusbench-sdbus-client',
  'sdbus-client.c',
  c_args: [],
  dependencies: dependency('libsystemd'),
  install: true,
  install_dir: get_option('bindir'))

install_data(
  'run-dbus-benchmark.sh',
  install_mode: 'rwxr-xr-x',
  install_dir: get_option('bindir'))
