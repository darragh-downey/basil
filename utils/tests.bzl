package(default_visibility = ["//visibility:public"])

load("@rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "greeter",
    srcs = ["utils/greeter_lib/srcs/lib.rs"],
)

rust_library(
    name = "reader",
    srcs = ["utils/reader_lib/srcs/lib.rs"],
)

load("@rules_rust//rust:defs.bzl", "rust_test_suite")

rust_test_suite(
    name = "integrated_test_suite",
    srcs = glob(["tests/**"]),
    deps = [
        ":greeter", 
        ":reader"
        ],
)