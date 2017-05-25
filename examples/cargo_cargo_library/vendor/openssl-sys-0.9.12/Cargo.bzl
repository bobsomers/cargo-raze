"""
cargo-raze generated details for openssl-sys-0.9.12.

Generated for:
platform_triple: x86_64-unknown-linux-gnu
platform_attrs:
[
    "debug_assertions",
    "target_arch: x86_64",
    "target_endian: little",
    "target_env: gnu",
    "target_family: unix",
    "target_feature: sse",
    "target_feature: sse2",
    "target_has_atomic: 16",
    "target_has_atomic: 32",
    "target_has_atomic: 64",
    "target_has_atomic: 8",
    "target_has_atomic: ptr",
    "target_os: linux",
    "target_pointer_width: 64",
    "target_thread_local",
    "target_vendor: unknown",
    "unix"
]

DO NOT MODIFY! Instead, add a CargoOverride.bzl mixin.
"""
description = struct(
    package = struct(
        pkg_name = "openssl-sys",
        pkg_version = "0.9.12",
    ),
    dependencies = [
        struct(
            name = "libc",
            version = "0.2.23",
        ),
    ],
    build_dependencies = [
        struct(
            name = "gcc",
            version = "0.3.46",
        ),
        struct(
            name = "pkg-config",
            version = "0.3.9",
        ),
    ],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "openssl-sys",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "build-script-build",
            kinds = [
                "custom-build",
            ],
            path = "build.rs",
        ),
    ],
)
