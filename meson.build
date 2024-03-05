project(
    'c-dbus-bench',
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
  'sdbusplus-server',
  'sdbusplus-server.cpp',
  cpp_args: cpp_args,
  dependencies: deps,
  install: true,
  install_dir: get_option('bindir'))
