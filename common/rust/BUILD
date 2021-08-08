package(default_visibility = ["//visibility:public"])

load("@rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "malicious",
    crate_name = "malicious",
    srcs = glob(["**/*.rs"]),
    #crate_type = "dylib",
)

#cc_library(
#    name = "parsing_c_wrapper",
#    srcs = [":parsing"],
#)
