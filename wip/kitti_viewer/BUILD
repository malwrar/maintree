load("@rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
  name = "kitti_viewer",
  srcs = [
    "src/main.rs",
  ],
  deps = [
    "//common/rust:malicious",
    "//third_party/cargo:env_logger",
    "//third_party/cargo:kdtree",
    "//third_party/cargo:log",
  ],
)
