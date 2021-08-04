package(default_visibility = ["//visibility:public"])

load("@rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "rust_lib",
    crate_name = "parsing",
    srcs = glob(["src/*.rs"]),
    #crate_type = "dylib",
)

#cc_library(
#    name = "parsing_c_wrapper",
#    srcs = [":parsing"],
#)
