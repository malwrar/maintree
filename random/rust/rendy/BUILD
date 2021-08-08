load("@rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
  name = "triangle",
  srcs = [
    "src/bin/triangle.rs",
  ],
  deps = [
    "//third_party/cargo:lazy_static",
    "//third_party/cargo:rendy",
    "//third_party/cargo:winit",
  ],
)
