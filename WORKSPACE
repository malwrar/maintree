# Common root bazel workspace.
#
# Bazel usage: https://docs.bazel.build/versions/4.1.0/guide.html
workspace(name = "malwrar")

# External bazel dependencies
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
rust_repositories(version = "1.53.0", edition="2018")

# Additional rules for setting up cargo BUILD file generation w/
# `@cargo_raze//:raze`
load("//third_party/cargo:crates.bzl", "raze_fetch_remote_crates")
raze_fetch_remote_crates()

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "cargo_raze",
    sha256 = "c664e258ea79e7e4ec2f2b57bca8b1c37f11c8d5748e02b8224810da969eb681",
    strip_prefix = "cargo-raze-0.11.0",
    url = "https://github.com/google/cargo-raze/archive/v0.11.0.tar.gz",
)

load("@cargo_raze//:repositories.bzl", "cargo_raze_repositories")

cargo_raze_repositories()

load("@cargo_raze//:transitive_deps.bzl", "cargo_raze_transitive_deps")

cargo_raze_transitive_deps()


# Python rules (https://github.com/bazelbuild/rules_python)
http_archive(
    name = "rules_python",
    url = "https://github.com/bazelbuild/rules_python/releases/download/0.3.0/rules_python-0.3.0.tar.gz",
    sha256 = "934c9ceb552e84577b0faf1e5a2f0450314985b4d8712b2b70717dc679fdc01b",
)


# Common external dependencies
local_repository(
    name = "glslang",
    path = "third_party/misc/glslang",
)

local_repository(
    name = "spirv_headers",
    path = "third_party/misc/SPIRV-Headers",
)

local_repository(
    name = "spirv_tools",
    path = "third_party/misc/SPIRV-Tools",
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

# Needed for:
#   - rust shaderc-sys crate (shaderc bindings)
new_local_repository(
    name = "shaderc",
    path = "third_party/misc/shaderc",
    build_file = "BUILD",
)
