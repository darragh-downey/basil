## I was getting too fancy will likely delete soon


package(default_visibility = ["//visibility:public"])

licenses([
    "notice",  # See individual crates for specific licenses
])

# No targets defined

# Export file for Stardoc support
exports_files(
    [
        "greeter_lib",
        "reader_lib",
    ],
    visibility = ["//visibility:public"],
)