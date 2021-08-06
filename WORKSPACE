load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# Rust rules (https://bazelbuild.github.io/rules_rust/)
http_archive(
    name = "rules_rust",
    sha256 = "224ebaf1156b6f2d3680e5b8c25191e71483214957dfecd25d0f29b2f283283b",
    strip_prefix = "rules_rust-a814d859845c420fd105c629134c4a4cb47ba3f8",
    urls = [
        # `main` branch as of 2021-06-15
        "https://github.com/bazelbuild/rules_rust/archive/a814d859845c420fd105c629134c4a4cb47ba3f8.tar.gz",
    ],
)
load("@rules_rust//rust:repositories.bzl", "rust_repositories")
rust_repositories()

# Uncomment if we switch from Vendoring to Remote (https://github.com/google/cargo-raze/tree/master#using-existing-cargotoml)
#load("//third_party/cargo:crates.bzl", "raze_fetch_remote_crates")
#raze_fetch_remote_crates()

# spirv
local_repository(
    name = "spirv_headers",
    path = "third_party/misc/SPIRV-Headers",
)

local_repository(
    name = "com_google_effcee",
    path = "third_party/misc/effcee",
)

local_repository(
    name = "com_google_googletest",
    path = "third_party/misc/googletest",
)

local_repository(
    name = "com_googlesource_code_re2",
    path = "third_party/misc/re2",
)
