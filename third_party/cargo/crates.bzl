"""
@generated
cargo-raze generated Bazel file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""

load("@bazel_tools//tools/build_defs/repo:git.bzl", "new_git_repository")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")  # buildifier: disable=load

def raze_fetch_remote_crates():
    """This function defines a collection of repos and should be called in a WORKSPACE file"""
    maybe(
        http_archive,
        name = "raze__ab_glyph_rasterizer__0_1_4",
        url = "https://crates.io/api/v1/crates/ab_glyph_rasterizer/0.1.4/download",
        type = "tar.gz",
        sha256 = "d9fe5e32de01730eb1f6b7f5b51c17e03e2325bf40a74f754f04f130043affff",
        strip_prefix = "ab_glyph_rasterizer-0.1.4",
        build_file = Label("//third_party/cargo/remote:BUILD.ab_glyph_rasterizer-0.1.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__adler32__1_2_0",
        url = "https://crates.io/api/v1/crates/adler32/1.2.0/download",
        type = "tar.gz",
        sha256 = "aae1277d39aeec15cb388266ecc24b11c80469deae6067e17a1a7aa9e5c1f234",
        strip_prefix = "adler32-1.2.0",
        build_file = Label("//third_party/cargo/remote:BUILD.adler32-1.2.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__aho_corasick__0_7_15",
        url = "https://crates.io/api/v1/crates/aho-corasick/0.7.15/download",
        type = "tar.gz",
        sha256 = "7404febffaa47dac81aa44dba71523c9d069b1bdc50a77db41195149e17f68e5",
        strip_prefix = "aho-corasick-0.7.15",
        build_file = Label("//third_party/cargo/remote:BUILD.aho-corasick-0.7.15.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__andrew__0_3_1",
        url = "https://crates.io/api/v1/crates/andrew/0.3.1/download",
        type = "tar.gz",
        sha256 = "8c4afb09dd642feec8408e33f92f3ffc4052946f6b20f32fb99c1f58cd4fa7cf",
        strip_prefix = "andrew-0.3.1",
        build_file = Label("//third_party/cargo/remote:BUILD.andrew-0.3.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__approx__0_3_2",
        url = "https://crates.io/api/v1/crates/approx/0.3.2/download",
        type = "tar.gz",
        sha256 = "f0e60b75072ecd4168020818c0107f2857bb6c4e64252d8d3983f6263b40a5c3",
        strip_prefix = "approx-0.3.2",
        build_file = Label("//third_party/cargo/remote:BUILD.approx-0.3.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__atty__0_2_14",
        url = "https://crates.io/api/v1/crates/atty/0.2.14/download",
        type = "tar.gz",
        sha256 = "d9b39be18770d11421cdb1b9947a45dd3f37e93092cbf377614828a319d5fee8",
        strip_prefix = "atty-0.2.14",
        build_file = Label("//third_party/cargo/remote:BUILD.atty-0.2.14.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__autocfg__0_1_7",
        url = "https://crates.io/api/v1/crates/autocfg/0.1.7/download",
        type = "tar.gz",
        sha256 = "1d49d90015b3c36167a20fe2810c5cd875ad504b39cff3d4eae7977e6b7c1cb2",
        strip_prefix = "autocfg-0.1.7",
        build_file = Label("//third_party/cargo/remote:BUILD.autocfg-0.1.7.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__autocfg__1_0_1",
        url = "https://crates.io/api/v1/crates/autocfg/1.0.1/download",
        type = "tar.gz",
        sha256 = "cdb031dd78e28731d87d56cc8ffef4a8f36ca26c38fe2de700543e627f8a464a",
        strip_prefix = "autocfg-1.0.1",
        build_file = Label("//third_party/cargo/remote:BUILD.autocfg-1.0.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__base64__0_13_0",
        url = "https://crates.io/api/v1/crates/base64/0.13.0/download",
        type = "tar.gz",
        sha256 = "904dfeac50f3cdaba28fc6f57fdcddb75f49ed61346676a78c4ffe55877802fd",
        strip_prefix = "base64-0.13.0",
        build_file = Label("//third_party/cargo/remote:BUILD.base64-0.13.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__bitflags__1_2_1",
        url = "https://crates.io/api/v1/crates/bitflags/1.2.1/download",
        type = "tar.gz",
        sha256 = "cf1de2fe8c75bc145a2f577add951f8134889b4795d47466a54a5c846d691693",
        strip_prefix = "bitflags-1.2.1",
        build_file = Label("//third_party/cargo/remote:BUILD.bitflags-1.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__block__0_1_6",
        url = "https://crates.io/api/v1/crates/block/0.1.6/download",
        type = "tar.gz",
        sha256 = "0d8c1fef690941d3e7788d328517591fecc684c084084702d6ff1641e993699a",
        strip_prefix = "block-0.1.6",
        build_file = Label("//third_party/cargo/remote:BUILD.block-0.1.6.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__byteorder__1_4_3",
        url = "https://crates.io/api/v1/crates/byteorder/1.4.3/download",
        type = "tar.gz",
        sha256 = "14c189c53d098945499cdfa7ecc63567cf3886b3332b312a5b4585d8d3a6a610",
        strip_prefix = "byteorder-1.4.3",
        build_file = Label("//third_party/cargo/remote:BUILD.byteorder-1.4.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__calloop__0_6_5",
        url = "https://crates.io/api/v1/crates/calloop/0.6.5/download",
        type = "tar.gz",
        sha256 = "0b036167e76041694579972c28cf4877b4f92da222560ddb49008937b6a6727c",
        strip_prefix = "calloop-0.6.5",
        build_file = Label("//third_party/cargo/remote:BUILD.calloop-0.6.5.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__cc__1_0_69",
        url = "https://crates.io/api/v1/crates/cc/1.0.69/download",
        type = "tar.gz",
        sha256 = "e70cc2f62c6ce1868963827bd677764c62d07c3d9a3e1fb1177ee1a9ab199eb2",
        strip_prefix = "cc-1.0.69",
        build_file = Label("//third_party/cargo/remote:BUILD.cc-1.0.69.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__cfg_if__0_1_10",
        url = "https://crates.io/api/v1/crates/cfg-if/0.1.10/download",
        type = "tar.gz",
        sha256 = "4785bdd1c96b2a846b2bd7cc02e86b6b3dbf14e7e53446c4f54c92a361040822",
        strip_prefix = "cfg-if-0.1.10",
        build_file = Label("//third_party/cargo/remote:BUILD.cfg-if-0.1.10.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__cfg_if__1_0_0",
        url = "https://crates.io/api/v1/crates/cfg-if/1.0.0/download",
        type = "tar.gz",
        sha256 = "baf1de4339761588bc0619e3cbc0120ee582ebb74b53b4efbf79117bd2da40fd",
        strip_prefix = "cfg-if-1.0.0",
        build_file = Label("//third_party/cargo/remote:BUILD.cfg-if-1.0.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__cgmath__0_17_0",
        url = "https://crates.io/api/v1/crates/cgmath/0.17.0/download",
        type = "tar.gz",
        sha256 = "283944cdecc44bf0b8dd010ec9af888d3b4f142844fdbe026c20ef68148d6fe7",
        strip_prefix = "cgmath-0.17.0",
        build_file = Label("//third_party/cargo/remote:BUILD.cgmath-0.17.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__cloudabi__0_0_3",
        url = "https://crates.io/api/v1/crates/cloudabi/0.0.3/download",
        type = "tar.gz",
        sha256 = "ddfc5b9aa5d4507acaf872de71051dfd0e309860e88966e1051e462a077aac4f",
        strip_prefix = "cloudabi-0.0.3",
        build_file = Label("//third_party/cargo/remote:BUILD.cloudabi-0.0.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__cmake__0_1_45",
        url = "https://crates.io/api/v1/crates/cmake/0.1.45/download",
        type = "tar.gz",
        sha256 = "eb6210b637171dfba4cda12e579ac6dc73f5165ad56133e5d72ef3131f320855",
        strip_prefix = "cmake-0.1.45",
        build_file = Label("//third_party/cargo/remote:BUILD.cmake-0.1.45.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__cocoa__0_20_2",
        url = "https://crates.io/api/v1/crates/cocoa/0.20.2/download",
        type = "tar.gz",
        sha256 = "0c49e86fc36d5704151f5996b7b3795385f50ce09e3be0f47a0cfde869681cf8",
        strip_prefix = "cocoa-0.20.2",
        build_file = Label("//third_party/cargo/remote:BUILD.cocoa-0.20.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__cocoa__0_24_0",
        url = "https://crates.io/api/v1/crates/cocoa/0.24.0/download",
        type = "tar.gz",
        sha256 = "6f63902e9223530efb4e26ccd0cf55ec30d592d3b42e21a28defc42a9586e832",
        strip_prefix = "cocoa-0.24.0",
        build_file = Label("//third_party/cargo/remote:BUILD.cocoa-0.24.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__cocoa_foundation__0_1_0",
        url = "https://crates.io/api/v1/crates/cocoa-foundation/0.1.0/download",
        type = "tar.gz",
        sha256 = "7ade49b65d560ca58c403a479bb396592b155c0185eada742ee323d1d68d6318",
        strip_prefix = "cocoa-foundation-0.1.0",
        build_file = Label("//third_party/cargo/remote:BUILD.cocoa-foundation-0.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__core_foundation__0_7_0",
        url = "https://crates.io/api/v1/crates/core-foundation/0.7.0/download",
        type = "tar.gz",
        sha256 = "57d24c7a13c43e870e37c1556b74555437870a04514f7685f5b354e090567171",
        strip_prefix = "core-foundation-0.7.0",
        build_file = Label("//third_party/cargo/remote:BUILD.core-foundation-0.7.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__core_foundation__0_9_1",
        url = "https://crates.io/api/v1/crates/core-foundation/0.9.1/download",
        type = "tar.gz",
        sha256 = "0a89e2ae426ea83155dccf10c0fa6b1463ef6d5fcb44cee0b224a408fa640a62",
        strip_prefix = "core-foundation-0.9.1",
        build_file = Label("//third_party/cargo/remote:BUILD.core-foundation-0.9.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__core_foundation_sys__0_7_0",
        url = "https://crates.io/api/v1/crates/core-foundation-sys/0.7.0/download",
        type = "tar.gz",
        sha256 = "b3a71ab494c0b5b860bdc8407ae08978052417070c2ced38573a9157ad75b8ac",
        strip_prefix = "core-foundation-sys-0.7.0",
        build_file = Label("//third_party/cargo/remote:BUILD.core-foundation-sys-0.7.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__core_foundation_sys__0_8_2",
        url = "https://crates.io/api/v1/crates/core-foundation-sys/0.8.2/download",
        type = "tar.gz",
        sha256 = "ea221b5284a47e40033bf9b66f35f984ec0ea2931eb03505246cd27a963f981b",
        strip_prefix = "core-foundation-sys-0.8.2",
        build_file = Label("//third_party/cargo/remote:BUILD.core-foundation-sys-0.8.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__core_graphics__0_19_2",
        url = "https://crates.io/api/v1/crates/core-graphics/0.19.2/download",
        type = "tar.gz",
        sha256 = "b3889374e6ea6ab25dba90bb5d96202f61108058361f6dc72e8b03e6f8bbe923",
        strip_prefix = "core-graphics-0.19.2",
        build_file = Label("//third_party/cargo/remote:BUILD.core-graphics-0.19.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__core_graphics__0_22_2",
        url = "https://crates.io/api/v1/crates/core-graphics/0.22.2/download",
        type = "tar.gz",
        sha256 = "269f35f69b542b80e736a20a89a05215c0ce80c2c03c514abb2e318b78379d86",
        strip_prefix = "core-graphics-0.22.2",
        build_file = Label("//third_party/cargo/remote:BUILD.core-graphics-0.22.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__core_graphics_types__0_1_1",
        url = "https://crates.io/api/v1/crates/core-graphics-types/0.1.1/download",
        type = "tar.gz",
        sha256 = "3a68b68b3446082644c91ac778bf50cd4104bfb002b5a6a7c44cca5a2c70788b",
        strip_prefix = "core-graphics-types-0.1.1",
        build_file = Label("//third_party/cargo/remote:BUILD.core-graphics-types-0.1.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__core_video_sys__0_1_4",
        url = "https://crates.io/api/v1/crates/core-video-sys/0.1.4/download",
        type = "tar.gz",
        sha256 = "34ecad23610ad9757664d644e369246edde1803fcb43ed72876565098a5d3828",
        strip_prefix = "core-video-sys-0.1.4",
        build_file = Label("//third_party/cargo/remote:BUILD.core-video-sys-0.1.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__crc32fast__1_2_1",
        url = "https://crates.io/api/v1/crates/crc32fast/1.2.1/download",
        type = "tar.gz",
        sha256 = "81156fece84ab6a9f2afdb109ce3ae577e42b1228441eded99bd77f627953b1a",
        strip_prefix = "crc32fast-1.2.1",
        build_file = Label("//third_party/cargo/remote:BUILD.crc32fast-1.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__crossbeam__0_7_3",
        url = "https://crates.io/api/v1/crates/crossbeam/0.7.3/download",
        type = "tar.gz",
        sha256 = "69323bff1fb41c635347b8ead484a5ca6c3f11914d784170b158d8449ab07f8e",
        strip_prefix = "crossbeam-0.7.3",
        build_file = Label("//third_party/cargo/remote:BUILD.crossbeam-0.7.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__crossbeam_channel__0_4_4",
        url = "https://crates.io/api/v1/crates/crossbeam-channel/0.4.4/download",
        type = "tar.gz",
        sha256 = "b153fe7cbef478c567df0f972e02e6d736db11affe43dfc9c56a9374d1adfb87",
        strip_prefix = "crossbeam-channel-0.4.4",
        build_file = Label("//third_party/cargo/remote:BUILD.crossbeam-channel-0.4.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__crossbeam_deque__0_7_4",
        url = "https://crates.io/api/v1/crates/crossbeam-deque/0.7.4/download",
        type = "tar.gz",
        sha256 = "c20ff29ded3204c5106278a81a38f4b482636ed4fa1e6cfbeef193291beb29ed",
        strip_prefix = "crossbeam-deque-0.7.4",
        build_file = Label("//third_party/cargo/remote:BUILD.crossbeam-deque-0.7.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__crossbeam_epoch__0_8_2",
        url = "https://crates.io/api/v1/crates/crossbeam-epoch/0.8.2/download",
        type = "tar.gz",
        sha256 = "058ed274caafc1f60c4997b5fc07bf7dc7cca454af7c6e81edffe5f33f70dace",
        strip_prefix = "crossbeam-epoch-0.8.2",
        build_file = Label("//third_party/cargo/remote:BUILD.crossbeam-epoch-0.8.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__crossbeam_queue__0_2_3",
        url = "https://crates.io/api/v1/crates/crossbeam-queue/0.2.3/download",
        type = "tar.gz",
        sha256 = "774ba60a54c213d409d5353bda12d49cd68d14e45036a285234c8d6f91f92570",
        strip_prefix = "crossbeam-queue-0.2.3",
        build_file = Label("//third_party/cargo/remote:BUILD.crossbeam-queue-0.2.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__crossbeam_utils__0_7_2",
        url = "https://crates.io/api/v1/crates/crossbeam-utils/0.7.2/download",
        type = "tar.gz",
        sha256 = "c3c7c73a2d1e9fc0886a08b93e98eb643461230d5f1925e4036204d5f2e261a8",
        strip_prefix = "crossbeam-utils-0.7.2",
        build_file = Label("//third_party/cargo/remote:BUILD.crossbeam-utils-0.7.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__darling__0_10_2",
        url = "https://crates.io/api/v1/crates/darling/0.10.2/download",
        type = "tar.gz",
        sha256 = "0d706e75d87e35569db781a9b5e2416cff1236a47ed380831f959382ccd5f858",
        strip_prefix = "darling-0.10.2",
        build_file = Label("//third_party/cargo/remote:BUILD.darling-0.10.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__darling_core__0_10_2",
        url = "https://crates.io/api/v1/crates/darling_core/0.10.2/download",
        type = "tar.gz",
        sha256 = "f0c960ae2da4de88a91b2d920c2a7233b400bc33cb28453a2987822d8392519b",
        strip_prefix = "darling_core-0.10.2",
        build_file = Label("//third_party/cargo/remote:BUILD.darling_core-0.10.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__darling_macro__0_10_2",
        url = "https://crates.io/api/v1/crates/darling_macro/0.10.2/download",
        type = "tar.gz",
        sha256 = "d9b5a2f4ac4969822c62224815d069952656cadc7084fdca9751e6d959189b72",
        strip_prefix = "darling_macro-0.10.2",
        build_file = Label("//third_party/cargo/remote:BUILD.darling_macro-0.10.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__deflate__0_7_20",
        url = "https://crates.io/api/v1/crates/deflate/0.7.20/download",
        type = "tar.gz",
        sha256 = "707b6a7b384888a70c8d2e8650b3e60170dfc6a67bb4aa67b6dfca57af4bedb4",
        strip_prefix = "deflate-0.7.20",
        build_file = Label("//third_party/cargo/remote:BUILD.deflate-0.7.20.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__derivative__2_2_0",
        url = "https://crates.io/api/v1/crates/derivative/2.2.0/download",
        type = "tar.gz",
        sha256 = "fcc3dd5e9e9c0b295d6e1e4d811fb6f157d5ffd784b8d202fc62eac8035a770b",
        strip_prefix = "derivative-2.2.0",
        build_file = Label("//third_party/cargo/remote:BUILD.derivative-2.2.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__dispatch__0_2_0",
        url = "https://crates.io/api/v1/crates/dispatch/0.2.0/download",
        type = "tar.gz",
        sha256 = "bd0c93bb4b0c6d9b77f4435b0ae98c24d17f1c45b2ff844c6151a07256ca923b",
        strip_prefix = "dispatch-0.2.0",
        build_file = Label("//third_party/cargo/remote:BUILD.dispatch-0.2.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__dlib__0_4_2",
        url = "https://crates.io/api/v1/crates/dlib/0.4.2/download",
        type = "tar.gz",
        sha256 = "b11f15d1e3268f140f68d390637d5e76d849782d971ae7063e0da69fe9709a76",
        strip_prefix = "dlib-0.4.2",
        build_file = Label("//third_party/cargo/remote:BUILD.dlib-0.4.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__dlib__0_5_0",
        url = "https://crates.io/api/v1/crates/dlib/0.5.0/download",
        type = "tar.gz",
        sha256 = "ac1b7517328c04c2aa68422fc60a41b92208182142ed04a25879c26c8f878794",
        strip_prefix = "dlib-0.5.0",
        build_file = Label("//third_party/cargo/remote:BUILD.dlib-0.5.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__downcast_rs__1_2_0",
        url = "https://crates.io/api/v1/crates/downcast-rs/1.2.0/download",
        type = "tar.gz",
        sha256 = "9ea835d29036a4087793836fa931b08837ad5e957da9e23886b29586fb9b6650",
        strip_prefix = "downcast-rs-1.2.0",
        build_file = Label("//third_party/cargo/remote:BUILD.downcast-rs-1.2.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__env_logger__0_8_4",
        url = "https://crates.io/api/v1/crates/env_logger/0.8.4/download",
        type = "tar.gz",
        sha256 = "a19187fea3ac7e84da7dacf48de0c45d63c6a76f9490dae389aead16c243fce3",
        strip_prefix = "env_logger-0.8.4",
        build_file = Label("//third_party/cargo/remote:BUILD.env_logger-0.8.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__fnv__1_0_7",
        url = "https://crates.io/api/v1/crates/fnv/1.0.7/download",
        type = "tar.gz",
        sha256 = "3f9eec918d3f24069decb9af1554cad7c880e2da24a9afd88aca000531ab82c1",
        strip_prefix = "fnv-1.0.7",
        build_file = Label("//third_party/cargo/remote:BUILD.fnv-1.0.7.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__foreign_types__0_3_2",
        url = "https://crates.io/api/v1/crates/foreign-types/0.3.2/download",
        type = "tar.gz",
        sha256 = "f6f339eb8adc052cd2ca78910fda869aefa38d22d5cb648e6485e4d3fc06f3b1",
        strip_prefix = "foreign-types-0.3.2",
        build_file = Label("//third_party/cargo/remote:BUILD.foreign-types-0.3.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__foreign_types_shared__0_1_1",
        url = "https://crates.io/api/v1/crates/foreign-types-shared/0.1.1/download",
        type = "tar.gz",
        sha256 = "00b0228411908ca8685dba7fc2cdd70ec9990a6e753e89b6ac91a84c40fbaf4b",
        strip_prefix = "foreign-types-shared-0.1.1",
        build_file = Label("//third_party/cargo/remote:BUILD.foreign-types-shared-0.1.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__fuchsia_cprng__0_1_1",
        url = "https://crates.io/api/v1/crates/fuchsia-cprng/0.1.1/download",
        type = "tar.gz",
        sha256 = "a06f77d526c1a601b7c4cdd98f54b5eaabffc14d5f2f0296febdc7f357c6d3ba",
        strip_prefix = "fuchsia-cprng-0.1.1",
        build_file = Label("//third_party/cargo/remote:BUILD.fuchsia-cprng-0.1.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__fuchsia_zircon__0_3_3",
        url = "https://crates.io/api/v1/crates/fuchsia-zircon/0.3.3/download",
        type = "tar.gz",
        sha256 = "2e9763c69ebaae630ba35f74888db465e49e259ba1bc0eda7d06f4a067615d82",
        strip_prefix = "fuchsia-zircon-0.3.3",
        build_file = Label("//third_party/cargo/remote:BUILD.fuchsia-zircon-0.3.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__fuchsia_zircon_sys__0_3_3",
        url = "https://crates.io/api/v1/crates/fuchsia-zircon-sys/0.3.3/download",
        type = "tar.gz",
        sha256 = "3dcaa9ae7725d12cdb85b3ad99a434db70b468c09ded17e012d86b5c1010f7a7",
        strip_prefix = "fuchsia-zircon-sys-0.3.3",
        build_file = Label("//third_party/cargo/remote:BUILD.fuchsia-zircon-sys-0.3.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__half__1_7_1",
        url = "https://crates.io/api/v1/crates/half/1.7.1/download",
        type = "tar.gz",
        sha256 = "62aca2aba2d62b4a7f5b33f3712cb1b0692779a56fb510499d5c0aa594daeaf3",
        strip_prefix = "half-1.7.1",
        build_file = Label("//third_party/cargo/remote:BUILD.half-1.7.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__hermit_abi__0_1_19",
        url = "https://crates.io/api/v1/crates/hermit-abi/0.1.19/download",
        type = "tar.gz",
        sha256 = "62b467343b94ba476dcb2500d242dadbb39557df889310ac77c5d99100aaac33",
        strip_prefix = "hermit-abi-0.1.19",
        build_file = Label("//third_party/cargo/remote:BUILD.hermit-abi-0.1.19.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__humantime__2_1_0",
        url = "https://crates.io/api/v1/crates/humantime/2.1.0/download",
        type = "tar.gz",
        sha256 = "9a3a5bfb195931eeb336b2a7b4d761daec841b97f947d34394601737a7bba5e4",
        strip_prefix = "humantime-2.1.0",
        build_file = Label("//third_party/cargo/remote:BUILD.humantime-2.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__ident_case__1_0_1",
        url = "https://crates.io/api/v1/crates/ident_case/1.0.1/download",
        type = "tar.gz",
        sha256 = "b9e0384b61958566e926dc50660321d12159025e767c18e043daf26b70104c39",
        strip_prefix = "ident_case-1.0.1",
        build_file = Label("//third_party/cargo/remote:BUILD.ident_case-1.0.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__inflate__0_4_5",
        url = "https://crates.io/api/v1/crates/inflate/0.4.5/download",
        type = "tar.gz",
        sha256 = "1cdb29978cc5797bd8dcc8e5bf7de604891df2a8dc576973d71a281e916db2ff",
        strip_prefix = "inflate-0.4.5",
        build_file = Label("//third_party/cargo/remote:BUILD.inflate-0.4.5.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__instant__0_1_10",
        url = "https://crates.io/api/v1/crates/instant/0.1.10/download",
        type = "tar.gz",
        sha256 = "bee0328b1209d157ef001c94dd85b4f8f64139adb0eac2659f4b08382b2f474d",
        strip_prefix = "instant-0.1.10",
        build_file = Label("//third_party/cargo/remote:BUILD.instant-0.1.10.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__iovec__0_1_4",
        url = "https://crates.io/api/v1/crates/iovec/0.1.4/download",
        type = "tar.gz",
        sha256 = "b2b3ea6ff95e175473f8ffe6a7eb7c00d054240321b84c57051175fe3c1e075e",
        strip_prefix = "iovec-0.1.4",
        build_file = Label("//third_party/cargo/remote:BUILD.iovec-0.1.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__jni_sys__0_3_0",
        url = "https://crates.io/api/v1/crates/jni-sys/0.3.0/download",
        type = "tar.gz",
        sha256 = "8eaf4bc02d17cbdd7ff4c7438cafcdf7fb9a4613313ad11b4f8fefe7d3fa0130",
        strip_prefix = "jni-sys-0.3.0",
        build_file = Label("//third_party/cargo/remote:BUILD.jni-sys-0.3.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__kdtree__0_6_0",
        url = "https://crates.io/api/v1/crates/kdtree/0.6.0/download",
        type = "tar.gz",
        sha256 = "80ee359328fc9087e9e3fc0a4567c4dd27ec69a127d6a70e8d9dd22845b8b1a2",
        strip_prefix = "kdtree-0.6.0",
        build_file = Label("//third_party/cargo/remote:BUILD.kdtree-0.6.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__kernel32_sys__0_2_2",
        url = "https://crates.io/api/v1/crates/kernel32-sys/0.2.2/download",
        type = "tar.gz",
        sha256 = "7507624b29483431c0ba2d82aece8ca6cdba9382bff4ddd0f7490560c056098d",
        strip_prefix = "kernel32-sys-0.2.2",
        build_file = Label("//third_party/cargo/remote:BUILD.kernel32-sys-0.2.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__lazy_static__1_4_0",
        url = "https://crates.io/api/v1/crates/lazy_static/1.4.0/download",
        type = "tar.gz",
        sha256 = "e2abad23fbc42b3700f2f279844dc832adb2b2eb069b2df918f455c4e18cc646",
        strip_prefix = "lazy_static-1.4.0",
        build_file = Label("//third_party/cargo/remote:BUILD.lazy_static-1.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__lazycell__1_3_0",
        url = "https://crates.io/api/v1/crates/lazycell/1.3.0/download",
        type = "tar.gz",
        sha256 = "830d08ce1d1d941e6b30645f1a0eb5643013d835ce3779a5fc208261dbe10f55",
        strip_prefix = "lazycell-1.3.0",
        build_file = Label("//third_party/cargo/remote:BUILD.lazycell-1.3.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__libc__0_2_98",
        url = "https://crates.io/api/v1/crates/libc/0.2.98/download",
        type = "tar.gz",
        sha256 = "320cfe77175da3a483efed4bc0adc1968ca050b098ce4f2f1c13a56626128790",
        strip_prefix = "libc-0.2.98",
        build_file = Label("//third_party/cargo/remote:BUILD.libc-0.2.98.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__libloading__0_6_7",
        url = "https://crates.io/api/v1/crates/libloading/0.6.7/download",
        type = "tar.gz",
        sha256 = "351a32417a12d5f7e82c368a66781e307834dae04c6ce0cd4456d52989229883",
        strip_prefix = "libloading-0.6.7",
        build_file = Label("//third_party/cargo/remote:BUILD.libloading-0.6.7.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__libloading__0_7_0",
        url = "https://crates.io/api/v1/crates/libloading/0.7.0/download",
        type = "tar.gz",
        sha256 = "6f84d96438c15fcd6c3f244c8fce01d1e2b9c6b5623e9c711dc9286d8fc92d6a",
        strip_prefix = "libloading-0.7.0",
        build_file = Label("//third_party/cargo/remote:BUILD.libloading-0.7.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__lock_api__0_4_4",
        url = "https://crates.io/api/v1/crates/lock_api/0.4.4/download",
        type = "tar.gz",
        sha256 = "0382880606dff6d15c9476c416d18690b72742aa7b605bb6dd6ec9030fbf07eb",
        strip_prefix = "lock_api-0.4.4",
        build_file = Label("//third_party/cargo/remote:BUILD.lock_api-0.4.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__log__0_4_14",
        url = "https://crates.io/api/v1/crates/log/0.4.14/download",
        type = "tar.gz",
        sha256 = "51b9bbe6c47d51fc3e1a9b945965946b4c44142ab8792c50835a980d362c2710",
        strip_prefix = "log-0.4.14",
        build_file = Label("//third_party/cargo/remote:BUILD.log-0.4.14.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__malloc_buf__0_0_6",
        url = "https://crates.io/api/v1/crates/malloc_buf/0.0.6/download",
        type = "tar.gz",
        sha256 = "62bb907fe88d54d8d9ce32a3cceab4218ed2f6b7d35617cafe9adf84e43919cb",
        strip_prefix = "malloc_buf-0.0.6",
        build_file = Label("//third_party/cargo/remote:BUILD.malloc_buf-0.0.6.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__maybe_uninit__2_0_0",
        url = "https://crates.io/api/v1/crates/maybe-uninit/2.0.0/download",
        type = "tar.gz",
        sha256 = "60302e4db3a61da70c0cb7991976248362f30319e88850c487b9b95bbf059e00",
        strip_prefix = "maybe-uninit-2.0.0",
        build_file = Label("//third_party/cargo/remote:BUILD.maybe-uninit-2.0.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__memchr__2_3_4",
        url = "https://crates.io/api/v1/crates/memchr/2.3.4/download",
        type = "tar.gz",
        sha256 = "0ee1c47aaa256ecabcaea351eae4a9b01ef39ed810004e298d2511ed284b1525",
        strip_prefix = "memchr-2.3.4",
        build_file = Label("//third_party/cargo/remote:BUILD.memchr-2.3.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__memmap2__0_1_0",
        url = "https://crates.io/api/v1/crates/memmap2/0.1.0/download",
        type = "tar.gz",
        sha256 = "d9b70ca2a6103ac8b665dc150b142ef0e4e89df640c9e6cf295d189c3caebe5a",
        strip_prefix = "memmap2-0.1.0",
        build_file = Label("//third_party/cargo/remote:BUILD.memmap2-0.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__memoffset__0_5_6",
        url = "https://crates.io/api/v1/crates/memoffset/0.5.6/download",
        type = "tar.gz",
        sha256 = "043175f069eda7b85febe4a74abbaeff828d9f8b448515d3151a14a3542811aa",
        strip_prefix = "memoffset-0.5.6",
        build_file = Label("//third_party/cargo/remote:BUILD.memoffset-0.5.6.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__metal__0_18_0",
        url = "https://crates.io/api/v1/crates/metal/0.18.0/download",
        type = "tar.gz",
        sha256 = "e198a0ee42bdbe9ef2c09d0b9426f3b2b47d90d93a4a9b0395c4cea605e92dc0",
        strip_prefix = "metal-0.18.0",
        build_file = Label("//third_party/cargo/remote:BUILD.metal-0.18.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__mio__0_6_23",
        url = "https://crates.io/api/v1/crates/mio/0.6.23/download",
        type = "tar.gz",
        sha256 = "4afd66f5b91bf2a3bc13fad0e21caedac168ca4c707504e75585648ae80e4cc4",
        strip_prefix = "mio-0.6.23",
        build_file = Label("//third_party/cargo/remote:BUILD.mio-0.6.23.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__mio_extras__2_0_6",
        url = "https://crates.io/api/v1/crates/mio-extras/2.0.6/download",
        type = "tar.gz",
        sha256 = "52403fe290012ce777c4626790c8951324a2b9e3316b3143779c72b029742f19",
        strip_prefix = "mio-extras-2.0.6",
        build_file = Label("//third_party/cargo/remote:BUILD.mio-extras-2.0.6.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__miow__0_2_2",
        url = "https://crates.io/api/v1/crates/miow/0.2.2/download",
        type = "tar.gz",
        sha256 = "ebd808424166322d4a38da87083bfddd3ac4c131334ed55856112eb06d46944d",
        strip_prefix = "miow-0.2.2",
        build_file = Label("//third_party/cargo/remote:BUILD.miow-0.2.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__ndk__0_2_1",
        url = "https://crates.io/api/v1/crates/ndk/0.2.1/download",
        type = "tar.gz",
        sha256 = "5eb167c1febed0a496639034d0c76b3b74263636045db5489eee52143c246e73",
        strip_prefix = "ndk-0.2.1",
        build_file = Label("//third_party/cargo/remote:BUILD.ndk-0.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__ndk_glue__0_2_1",
        url = "https://crates.io/api/v1/crates/ndk-glue/0.2.1/download",
        type = "tar.gz",
        sha256 = "bdf399b8b7a39c6fb153c4ec32c72fd5fe789df24a647f229c239aa7adb15241",
        strip_prefix = "ndk-glue-0.2.1",
        build_file = Label("//third_party/cargo/remote:BUILD.ndk-glue-0.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__ndk_macro__0_2_0",
        url = "https://crates.io/api/v1/crates/ndk-macro/0.2.0/download",
        type = "tar.gz",
        sha256 = "05d1c6307dc424d0f65b9b06e94f88248e6305726b14729fd67a5e47b2dc481d",
        strip_prefix = "ndk-macro-0.2.0",
        build_file = Label("//third_party/cargo/remote:BUILD.ndk-macro-0.2.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__ndk_sys__0_2_1",
        url = "https://crates.io/api/v1/crates/ndk-sys/0.2.1/download",
        type = "tar.gz",
        sha256 = "c44922cb3dbb1c70b5e5f443d63b64363a898564d739ba5198e3a9138442868d",
        strip_prefix = "ndk-sys-0.2.1",
        build_file = Label("//third_party/cargo/remote:BUILD.ndk-sys-0.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__net2__0_2_37",
        url = "https://crates.io/api/v1/crates/net2/0.2.37/download",
        type = "tar.gz",
        sha256 = "391630d12b68002ae1e25e8f974306474966550ad82dac6886fb8910c19568ae",
        strip_prefix = "net2-0.2.37",
        build_file = Label("//third_party/cargo/remote:BUILD.net2-0.2.37.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__nix__0_18_0",
        url = "https://crates.io/api/v1/crates/nix/0.18.0/download",
        type = "tar.gz",
        sha256 = "83450fe6a6142ddd95fb064b746083fc4ef1705fe81f64a64e1d4b39f54a1055",
        strip_prefix = "nix-0.18.0",
        build_file = Label("//third_party/cargo/remote:BUILD.nix-0.18.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__nix__0_20_0",
        url = "https://crates.io/api/v1/crates/nix/0.20.0/download",
        type = "tar.gz",
        sha256 = "fa9b4819da1bc61c0ea48b63b7bc8604064dd43013e7cc325df098d49cd7c18a",
        strip_prefix = "nix-0.20.0",
        build_file = Label("//third_party/cargo/remote:BUILD.nix-0.20.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__nom__6_2_1",
        url = "https://crates.io/api/v1/crates/nom/6.2.1/download",
        type = "tar.gz",
        sha256 = "9c5c51b9083a3c620fa67a2a635d1ce7d95b897e957d6b28ff9a5da960a103a6",
        strip_prefix = "nom-6.2.1",
        build_file = Label("//third_party/cargo/remote:BUILD.nom-6.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__num_traits__0_2_14",
        url = "https://crates.io/api/v1/crates/num-traits/0.2.14/download",
        type = "tar.gz",
        sha256 = "9a64b1ec5cda2586e284722486d802acf1f7dbdc623e2bfc57e65ca1cd099290",
        strip_prefix = "num-traits-0.2.14",
        build_file = Label("//third_party/cargo/remote:BUILD.num-traits-0.2.14.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__num_enum__0_4_3",
        url = "https://crates.io/api/v1/crates/num_enum/0.4.3/download",
        type = "tar.gz",
        sha256 = "ca565a7df06f3d4b485494f25ba05da1435950f4dc263440eda7a6fa9b8e36e4",
        strip_prefix = "num_enum-0.4.3",
        build_file = Label("//third_party/cargo/remote:BUILD.num_enum-0.4.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__num_enum_derive__0_4_3",
        url = "https://crates.io/api/v1/crates/num_enum_derive/0.4.3/download",
        type = "tar.gz",
        sha256 = "ffa5a33ddddfee04c0283a7653987d634e880347e96b5b2ed64de07efb59db9d",
        strip_prefix = "num_enum_derive-0.4.3",
        build_file = Label("//third_party/cargo/remote:BUILD.num_enum_derive-0.4.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__objc__0_2_7",
        url = "https://crates.io/api/v1/crates/objc/0.2.7/download",
        type = "tar.gz",
        sha256 = "915b1b472bc21c53464d6c8461c9d3af805ba1ef837e1cac254428f4a77177b1",
        strip_prefix = "objc-0.2.7",
        build_file = Label("//third_party/cargo/remote:BUILD.objc-0.2.7.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__objc_exception__0_1_2",
        url = "https://crates.io/api/v1/crates/objc_exception/0.1.2/download",
        type = "tar.gz",
        sha256 = "ad970fb455818ad6cba4c122ad012fae53ae8b4795f86378bce65e4f6bab2ca4",
        strip_prefix = "objc_exception-0.1.2",
        build_file = Label("//third_party/cargo/remote:BUILD.objc_exception-0.1.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__once_cell__1_8_0",
        url = "https://crates.io/api/v1/crates/once_cell/1.8.0/download",
        type = "tar.gz",
        sha256 = "692fcb63b64b1758029e0a96ee63e049ce8c5948587f2f7208df04625e5f6b56",
        strip_prefix = "once_cell-1.8.0",
        build_file = Label("//third_party/cargo/remote:BUILD.once_cell-1.8.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__owned_ttf_parser__0_6_0",
        url = "https://crates.io/api/v1/crates/owned_ttf_parser/0.6.0/download",
        type = "tar.gz",
        sha256 = "9f923fb806c46266c02ab4a5b239735c144bdeda724a50ed058e5226f594cde3",
        strip_prefix = "owned_ttf_parser-0.6.0",
        build_file = Label("//third_party/cargo/remote:BUILD.owned_ttf_parser-0.6.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__parking_lot__0_11_1",
        url = "https://crates.io/api/v1/crates/parking_lot/0.11.1/download",
        type = "tar.gz",
        sha256 = "6d7744ac029df22dca6284efe4e898991d28e3085c706c972bcd7da4a27a15eb",
        strip_prefix = "parking_lot-0.11.1",
        build_file = Label("//third_party/cargo/remote:BUILD.parking_lot-0.11.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__parking_lot_core__0_8_3",
        url = "https://crates.io/api/v1/crates/parking_lot_core/0.8.3/download",
        type = "tar.gz",
        sha256 = "fa7a782938e745763fe6907fc6ba86946d72f49fe7e21de074e08128a99fb018",
        strip_prefix = "parking_lot_core-0.8.3",
        build_file = Label("//third_party/cargo/remote:BUILD.parking_lot_core-0.8.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__percent_encoding__2_1_0",
        url = "https://crates.io/api/v1/crates/percent-encoding/2.1.0/download",
        type = "tar.gz",
        sha256 = "d4fd5641d01c8f18a23da7b6fe29298ff4b55afcccdf78973b24cf3175fee32e",
        strip_prefix = "percent-encoding-2.1.0",
        build_file = Label("//third_party/cargo/remote:BUILD.percent-encoding-2.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__pkg_config__0_3_19",
        url = "https://crates.io/api/v1/crates/pkg-config/0.3.19/download",
        type = "tar.gz",
        sha256 = "3831453b3449ceb48b6d9c7ad7c96d5ea673e9b470a1dc578c2ce6521230884c",
        strip_prefix = "pkg-config-0.3.19",
        build_file = Label("//third_party/cargo/remote:BUILD.pkg-config-0.3.19.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__png__0_15_3",
        url = "https://crates.io/api/v1/crates/png/0.15.3/download",
        type = "tar.gz",
        sha256 = "ef859a23054bbfee7811284275ae522f0434a3c8e7f4b74bd4a35ae7e1c4a283",
        strip_prefix = "png-0.15.3",
        build_file = Label("//third_party/cargo/remote:BUILD.png-0.15.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__proc_macro_crate__0_1_5",
        url = "https://crates.io/api/v1/crates/proc-macro-crate/0.1.5/download",
        type = "tar.gz",
        sha256 = "1d6ea3c4595b96363c13943497db34af4460fb474a95c43f4446ad341b8c9785",
        strip_prefix = "proc-macro-crate-0.1.5",
        build_file = Label("//third_party/cargo/remote:BUILD.proc-macro-crate-0.1.5.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__proc_macro2__1_0_28",
        url = "https://crates.io/api/v1/crates/proc-macro2/1.0.28/download",
        type = "tar.gz",
        sha256 = "5c7ed8b8c7b886ea3ed7dde405212185f423ab44682667c8c6dd14aa1d9f6612",
        strip_prefix = "proc-macro2-1.0.28",
        build_file = Label("//third_party/cargo/remote:BUILD.proc-macro2-1.0.28.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__quote__1_0_9",
        url = "https://crates.io/api/v1/crates/quote/1.0.9/download",
        type = "tar.gz",
        sha256 = "c3d0b9745dc2debf507c8422de05d7226cc1f0644216dfdfead988f9b1ab32a7",
        strip_prefix = "quote-1.0.9",
        build_file = Label("//third_party/cargo/remote:BUILD.quote-1.0.9.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__rand__0_6_5",
        url = "https://crates.io/api/v1/crates/rand/0.6.5/download",
        type = "tar.gz",
        sha256 = "6d71dacdc3c88c1fde3885a3be3fbab9f35724e6ce99467f7d9c5026132184ca",
        strip_prefix = "rand-0.6.5",
        build_file = Label("//third_party/cargo/remote:BUILD.rand-0.6.5.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__rand_chacha__0_1_1",
        url = "https://crates.io/api/v1/crates/rand_chacha/0.1.1/download",
        type = "tar.gz",
        sha256 = "556d3a1ca6600bfcbab7c7c91ccb085ac7fbbcd70e008a98742e7847f4f7bcef",
        strip_prefix = "rand_chacha-0.1.1",
        build_file = Label("//third_party/cargo/remote:BUILD.rand_chacha-0.1.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__rand_core__0_3_1",
        url = "https://crates.io/api/v1/crates/rand_core/0.3.1/download",
        type = "tar.gz",
        sha256 = "7a6fdeb83b075e8266dcc8762c22776f6877a63111121f5f8c7411e5be7eed4b",
        strip_prefix = "rand_core-0.3.1",
        build_file = Label("//third_party/cargo/remote:BUILD.rand_core-0.3.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__rand_core__0_4_2",
        url = "https://crates.io/api/v1/crates/rand_core/0.4.2/download",
        type = "tar.gz",
        sha256 = "9c33a3c44ca05fa6f1807d8e6743f3824e8509beca625669633be0acbdf509dc",
        strip_prefix = "rand_core-0.4.2",
        build_file = Label("//third_party/cargo/remote:BUILD.rand_core-0.4.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__rand_hc__0_1_0",
        url = "https://crates.io/api/v1/crates/rand_hc/0.1.0/download",
        type = "tar.gz",
        sha256 = "7b40677c7be09ae76218dc623efbf7b18e34bced3f38883af07bb75630a21bc4",
        strip_prefix = "rand_hc-0.1.0",
        build_file = Label("//third_party/cargo/remote:BUILD.rand_hc-0.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__rand_isaac__0_1_1",
        url = "https://crates.io/api/v1/crates/rand_isaac/0.1.1/download",
        type = "tar.gz",
        sha256 = "ded997c9d5f13925be2a6fd7e66bf1872597f759fd9dd93513dd7e92e5a5ee08",
        strip_prefix = "rand_isaac-0.1.1",
        build_file = Label("//third_party/cargo/remote:BUILD.rand_isaac-0.1.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__rand_jitter__0_1_4",
        url = "https://crates.io/api/v1/crates/rand_jitter/0.1.4/download",
        type = "tar.gz",
        sha256 = "1166d5c91dc97b88d1decc3285bb0a99ed84b05cfd0bc2341bdf2d43fc41e39b",
        strip_prefix = "rand_jitter-0.1.4",
        build_file = Label("//third_party/cargo/remote:BUILD.rand_jitter-0.1.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__rand_os__0_1_3",
        url = "https://crates.io/api/v1/crates/rand_os/0.1.3/download",
        type = "tar.gz",
        sha256 = "7b75f676a1e053fc562eafbb47838d67c84801e38fc1ba459e8f180deabd5071",
        strip_prefix = "rand_os-0.1.3",
        build_file = Label("//third_party/cargo/remote:BUILD.rand_os-0.1.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__rand_pcg__0_1_2",
        url = "https://crates.io/api/v1/crates/rand_pcg/0.1.2/download",
        type = "tar.gz",
        sha256 = "abf9b09b01790cfe0364f52bf32995ea3c39f4d2dd011eac241d2914146d0b44",
        strip_prefix = "rand_pcg-0.1.2",
        build_file = Label("//third_party/cargo/remote:BUILD.rand_pcg-0.1.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__rand_xorshift__0_1_1",
        url = "https://crates.io/api/v1/crates/rand_xorshift/0.1.1/download",
        type = "tar.gz",
        sha256 = "cbf7e9e623549b0e21f6e97cf8ecf247c1a8fd2e8a992ae265314300b2455d5c",
        strip_prefix = "rand_xorshift-0.1.1",
        build_file = Label("//third_party/cargo/remote:BUILD.rand_xorshift-0.1.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__raw_window_handle__0_3_3",
        url = "https://crates.io/api/v1/crates/raw-window-handle/0.3.3/download",
        type = "tar.gz",
        sha256 = "0a441a7a6c80ad6473bd4b74ec1c9a4c951794285bf941c2126f607c72e48211",
        strip_prefix = "raw-window-handle-0.3.3",
        build_file = Label("//third_party/cargo/remote:BUILD.raw-window-handle-0.3.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__rdrand__0_4_0",
        url = "https://crates.io/api/v1/crates/rdrand/0.4.0/download",
        type = "tar.gz",
        sha256 = "678054eb77286b51581ba43620cc911abf02758c91f93f479767aed0f90458b2",
        strip_prefix = "rdrand-0.4.0",
        build_file = Label("//third_party/cargo/remote:BUILD.rdrand-0.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__redox_syscall__0_2_10",
        url = "https://crates.io/api/v1/crates/redox_syscall/0.2.10/download",
        type = "tar.gz",
        sha256 = "8383f39639269cde97d255a32bdb68c047337295414940c68bdd30c2e13203ff",
        strip_prefix = "redox_syscall-0.2.10",
        build_file = Label("//third_party/cargo/remote:BUILD.redox_syscall-0.2.10.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__regex__1_4_6",
        url = "https://crates.io/api/v1/crates/regex/1.4.6/download",
        type = "tar.gz",
        sha256 = "2a26af418b574bd56588335b3a3659a65725d4e636eb1016c2f9e3b38c7cc759",
        strip_prefix = "regex-1.4.6",
        build_file = Label("//third_party/cargo/remote:BUILD.regex-1.4.6.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__regex_syntax__0_6_25",
        url = "https://crates.io/api/v1/crates/regex-syntax/0.6.25/download",
        type = "tar.gz",
        sha256 = "f497285884f3fcff424ffc933e56d7cbca511def0c9831a7f9b5f6153e3cc89b",
        strip_prefix = "regex-syntax-0.6.25",
        build_file = Label("//third_party/cargo/remote:BUILD.regex-syntax-0.6.25.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__ron__0_6_4",
        url = "https://crates.io/api/v1/crates/ron/0.6.4/download",
        type = "tar.gz",
        sha256 = "064ea8613fb712a19faf920022ec8ddf134984f100090764a4e1d768f3827f1f",
        strip_prefix = "ron-0.6.4",
        build_file = Label("//third_party/cargo/remote:BUILD.ron-0.6.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__rusttype__0_9_2",
        url = "https://crates.io/api/v1/crates/rusttype/0.9.2/download",
        type = "tar.gz",
        sha256 = "dc7c727aded0be18c5b80c1640eae0ac8e396abf6fa8477d96cb37d18ee5ec59",
        strip_prefix = "rusttype-0.9.2",
        build_file = Label("//third_party/cargo/remote:BUILD.rusttype-0.9.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__same_file__1_0_6",
        url = "https://crates.io/api/v1/crates/same-file/1.0.6/download",
        type = "tar.gz",
        sha256 = "93fc1dc3aaa9bfed95e02e6eadabb4baf7e3078b0bd1b4d7b6b0b68378900502",
        strip_prefix = "same-file-1.0.6",
        build_file = Label("//third_party/cargo/remote:BUILD.same-file-1.0.6.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__scoped_tls__1_0_0",
        url = "https://crates.io/api/v1/crates/scoped-tls/1.0.0/download",
        type = "tar.gz",
        sha256 = "ea6a9290e3c9cf0f18145ef7ffa62d68ee0bf5fcd651017e586dc7fd5da448c2",
        strip_prefix = "scoped-tls-1.0.0",
        build_file = Label("//third_party/cargo/remote:BUILD.scoped-tls-1.0.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__scopeguard__1_1_0",
        url = "https://crates.io/api/v1/crates/scopeguard/1.1.0/download",
        type = "tar.gz",
        sha256 = "d29ab0c6d3fc0ee92fe66e2d99f700eab17a8d57d1c1d3b748380fb20baa78cd",
        strip_prefix = "scopeguard-1.1.0",
        build_file = Label("//third_party/cargo/remote:BUILD.scopeguard-1.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__serde__1_0_127",
        url = "https://crates.io/api/v1/crates/serde/1.0.127/download",
        type = "tar.gz",
        sha256 = "f03b9878abf6d14e6779d3f24f07b2cfa90352cfec4acc5aab8f1ac7f146fae8",
        strip_prefix = "serde-1.0.127",
        build_file = Label("//third_party/cargo/remote:BUILD.serde-1.0.127.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__serde_derive__1_0_127",
        url = "https://crates.io/api/v1/crates/serde_derive/1.0.127/download",
        type = "tar.gz",
        sha256 = "a024926d3432516606328597e0f224a51355a493b49fdd67e9209187cbe55ecc",
        strip_prefix = "serde_derive-1.0.127",
        build_file = Label("//third_party/cargo/remote:BUILD.serde_derive-1.0.127.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__shaderc__0_6_3",
        url = "https://crates.io/api/v1/crates/shaderc/0.6.3/download",
        type = "tar.gz",
        sha256 = "50b8aeaae10b9bda5cba66736a7e265f67698e912e1cc6a4678acba286e22be9",
        strip_prefix = "shaderc-0.6.3",
        build_file = Label("//third_party/cargo/remote:BUILD.shaderc-0.6.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__shaderc_sys__0_6_3",
        url = "https://crates.io/api/v1/crates/shaderc-sys/0.6.3/download",
        type = "tar.gz",
        sha256 = "5b12d7c62d6732884c9dfab587503fa3a795b108df152415a89da23812d4737e",
        strip_prefix = "shaderc-sys-0.6.3",
        build_file = Label("//third_party/cargo/remote:BUILD.shaderc-sys-0.6.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__shared_library__0_1_9",
        url = "https://crates.io/api/v1/crates/shared_library/0.1.9/download",
        type = "tar.gz",
        sha256 = "5a9e7e0f2bfae24d8a5b5a66c5b257a83c7412304311512a0c054cd5e619da11",
        strip_prefix = "shared_library-0.1.9",
        build_file = Label("//third_party/cargo/remote:BUILD.shared_library-0.1.9.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__slab__0_4_4",
        url = "https://crates.io/api/v1/crates/slab/0.4.4/download",
        type = "tar.gz",
        sha256 = "c307a32c1c5c437f38c7fd45d753050587732ba8628319fbdf12a7e289ccc590",
        strip_prefix = "slab-0.4.4",
        build_file = Label("//third_party/cargo/remote:BUILD.slab-0.4.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__smallvec__1_6_1",
        url = "https://crates.io/api/v1/crates/smallvec/1.6.1/download",
        type = "tar.gz",
        sha256 = "fe0f37c9e8f3c5a4a66ad655a93c74daac4ad00c441533bf5c6e7990bb42604e",
        strip_prefix = "smallvec-1.6.1",
        build_file = Label("//third_party/cargo/remote:BUILD.smallvec-1.6.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__smithay_client_toolkit__0_12_3",
        url = "https://crates.io/api/v1/crates/smithay-client-toolkit/0.12.3/download",
        type = "tar.gz",
        sha256 = "4750c76fd5d3ac95fa3ed80fe667d6a3d8590a960e5b575b98eea93339a80b80",
        strip_prefix = "smithay-client-toolkit-0.12.3",
        build_file = Label("//third_party/cargo/remote:BUILD.smithay-client-toolkit-0.12.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__strsim__0_9_3",
        url = "https://crates.io/api/v1/crates/strsim/0.9.3/download",
        type = "tar.gz",
        sha256 = "6446ced80d6c486436db5c078dde11a9f73d42b57fb273121e160b84f63d894c",
        strip_prefix = "strsim-0.9.3",
        build_file = Label("//third_party/cargo/remote:BUILD.strsim-0.9.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__syn__1_0_74",
        url = "https://crates.io/api/v1/crates/syn/1.0.74/download",
        type = "tar.gz",
        sha256 = "1873d832550d4588c3dbc20f01361ab00bfe741048f71e3fecf145a7cc18b29c",
        strip_prefix = "syn-1.0.74",
        build_file = Label("//third_party/cargo/remote:BUILD.syn-1.0.74.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__termcolor__1_1_2",
        url = "https://crates.io/api/v1/crates/termcolor/1.1.2/download",
        type = "tar.gz",
        sha256 = "2dfed899f0eb03f32ee8c6a0aabdb8a7949659e3466561fc0adf54e26d88c5f4",
        strip_prefix = "termcolor-1.1.2",
        build_file = Label("//third_party/cargo/remote:BUILD.termcolor-1.1.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__thiserror__1_0_26",
        url = "https://crates.io/api/v1/crates/thiserror/1.0.26/download",
        type = "tar.gz",
        sha256 = "93119e4feac1cbe6c798c34d3a53ea0026b0b1de6a120deef895137c0529bfe2",
        strip_prefix = "thiserror-1.0.26",
        build_file = Label("//third_party/cargo/remote:BUILD.thiserror-1.0.26.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__thiserror_impl__1_0_26",
        url = "https://crates.io/api/v1/crates/thiserror-impl/1.0.26/download",
        type = "tar.gz",
        sha256 = "060d69a0afe7796bf42e9e2ff91f5ee691fb15c53d38b4b62a9a53eb23164745",
        strip_prefix = "thiserror-impl-1.0.26",
        build_file = Label("//third_party/cargo/remote:BUILD.thiserror-impl-1.0.26.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__time__0_1_44",
        url = "https://crates.io/api/v1/crates/time/0.1.44/download",
        type = "tar.gz",
        sha256 = "6db9e6914ab8b1ae1c260a4ae7a49b6c5611b40328a735b21862567685e73255",
        strip_prefix = "time-0.1.44",
        build_file = Label("//third_party/cargo/remote:BUILD.time-0.1.44.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__toml__0_5_8",
        url = "https://crates.io/api/v1/crates/toml/0.5.8/download",
        type = "tar.gz",
        sha256 = "a31142970826733df8241ef35dc040ef98c679ab14d7c3e54d827099b3acecaa",
        strip_prefix = "toml-0.5.8",
        build_file = Label("//third_party/cargo/remote:BUILD.toml-0.5.8.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__ttf_parser__0_6_2",
        url = "https://crates.io/api/v1/crates/ttf-parser/0.6.2/download",
        type = "tar.gz",
        sha256 = "3e5d7cd7ab3e47dda6e56542f4bbf3824c15234958c6e1bd6aaa347e93499fdc",
        strip_prefix = "ttf-parser-0.6.2",
        build_file = Label("//third_party/cargo/remote:BUILD.ttf-parser-0.6.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__unicode_xid__0_2_2",
        url = "https://crates.io/api/v1/crates/unicode-xid/0.2.2/download",
        type = "tar.gz",
        sha256 = "8ccb82d61f80a663efe1f787a51b16b5a51e3314d6ac365b08639f52387b33f3",
        strip_prefix = "unicode-xid-0.2.2",
        build_file = Label("//third_party/cargo/remote:BUILD.unicode-xid-0.2.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__version_check__0_9_3",
        url = "https://crates.io/api/v1/crates/version_check/0.9.3/download",
        type = "tar.gz",
        sha256 = "5fecdca9a5291cc2b8dcf7dc02453fee791a280f3743cb0905f8822ae463b3fe",
        strip_prefix = "version_check-0.9.3",
        build_file = Label("//third_party/cargo/remote:BUILD.version_check-0.9.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__vk_sys__0_5_3",
        url = "https://crates.io/api/v1/crates/vk-sys/0.5.3/download",
        type = "tar.gz",
        sha256 = "712e7b00b858d5a65e4272e3dfd83f795a31467ba67425d853f32b966a09c907",
        strip_prefix = "vk-sys-0.5.3",
        build_file = Label("//third_party/cargo/remote:BUILD.vk-sys-0.5.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__vulkano__0_20_0",
        url = "https://crates.io/api/v1/crates/vulkano/0.20.0/download",
        type = "tar.gz",
        sha256 = "8f566b50e4f42000f48027dad19b38c41068657193feb59aa0b399664f5bf80d",
        strip_prefix = "vulkano-0.20.0",
        build_file = Label("//third_party/cargo/remote:BUILD.vulkano-0.20.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__vulkano_shaders__0_20_0",
        url = "https://crates.io/api/v1/crates/vulkano-shaders/0.20.0/download",
        type = "tar.gz",
        sha256 = "67d1d768877bf11729725bebec9e642b853207357abb89c239be598f0a1ddfbf",
        strip_prefix = "vulkano-shaders-0.20.0",
        build_file = Label("//third_party/cargo/remote:BUILD.vulkano-shaders-0.20.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__vulkano_win__0_20_0",
        url = "https://crates.io/api/v1/crates/vulkano-win/0.20.0/download",
        type = "tar.gz",
        sha256 = "5aa56544406650fb1472510fc75a28078316df9a27e4123504f080261e3c3406",
        strip_prefix = "vulkano-win-0.20.0",
        build_file = Label("//third_party/cargo/remote:BUILD.vulkano-win-0.20.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__walkdir__2_3_2",
        url = "https://crates.io/api/v1/crates/walkdir/2.3.2/download",
        type = "tar.gz",
        sha256 = "808cf2735cd4b6866113f648b791c6adc5714537bc222d9347bb203386ffda56",
        strip_prefix = "walkdir-2.3.2",
        build_file = Label("//third_party/cargo/remote:BUILD.walkdir-2.3.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wasi__0_10_0_wasi_snapshot_preview1",
        url = "https://crates.io/api/v1/crates/wasi/0.10.0+wasi-snapshot-preview1/download",
        type = "tar.gz",
        sha256 = "1a143597ca7c7793eff794def352d41792a93c481eb1042423ff7ff72ba2c31f",
        strip_prefix = "wasi-0.10.0+wasi-snapshot-preview1",
        build_file = Label("//third_party/cargo/remote:BUILD.wasi-0.10.0+wasi-snapshot-preview1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wayland_client__0_28_6",
        url = "https://crates.io/api/v1/crates/wayland-client/0.28.6/download",
        type = "tar.gz",
        sha256 = "e3ab332350e502f159382201394a78e3cc12d0f04db863429260164ea40e0355",
        strip_prefix = "wayland-client-0.28.6",
        build_file = Label("//third_party/cargo/remote:BUILD.wayland-client-0.28.6.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wayland_commons__0_28_6",
        url = "https://crates.io/api/v1/crates/wayland-commons/0.28.6/download",
        type = "tar.gz",
        sha256 = "a21817947c7011bbd0a27e11b17b337bfd022e8544b071a2641232047966fbda",
        strip_prefix = "wayland-commons-0.28.6",
        build_file = Label("//third_party/cargo/remote:BUILD.wayland-commons-0.28.6.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wayland_cursor__0_28_6",
        url = "https://crates.io/api/v1/crates/wayland-cursor/0.28.6/download",
        type = "tar.gz",
        sha256 = "be610084edd1586d45e7bdd275fe345c7c1873598caa464c4fb835dee70fa65a",
        strip_prefix = "wayland-cursor-0.28.6",
        build_file = Label("//third_party/cargo/remote:BUILD.wayland-cursor-0.28.6.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wayland_protocols__0_28_6",
        url = "https://crates.io/api/v1/crates/wayland-protocols/0.28.6/download",
        type = "tar.gz",
        sha256 = "286620ea4d803bacf61fa087a4242ee316693099ee5a140796aaba02b29f861f",
        strip_prefix = "wayland-protocols-0.28.6",
        build_file = Label("//third_party/cargo/remote:BUILD.wayland-protocols-0.28.6.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wayland_scanner__0_28_6",
        url = "https://crates.io/api/v1/crates/wayland-scanner/0.28.6/download",
        type = "tar.gz",
        sha256 = "ce923eb2deb61de332d1f356ec7b6bf37094dc5573952e1c8936db03b54c03f1",
        strip_prefix = "wayland-scanner-0.28.6",
        build_file = Label("//third_party/cargo/remote:BUILD.wayland-scanner-0.28.6.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wayland_sys__0_28_6",
        url = "https://crates.io/api/v1/crates/wayland-sys/0.28.6/download",
        type = "tar.gz",
        sha256 = "d841fca9aed7febf9bed2e9796c49bf58d4152ceda8ac949ebe00868d8f0feb8",
        strip_prefix = "wayland-sys-0.28.6",
        build_file = Label("//third_party/cargo/remote:BUILD.wayland-sys-0.28.6.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__winapi__0_2_8",
        url = "https://crates.io/api/v1/crates/winapi/0.2.8/download",
        type = "tar.gz",
        sha256 = "167dc9d6949a9b857f3451275e911c3f44255842c1f7a76f33c55103a909087a",
        strip_prefix = "winapi-0.2.8",
        build_file = Label("//third_party/cargo/remote:BUILD.winapi-0.2.8.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__winapi__0_3_9",
        url = "https://crates.io/api/v1/crates/winapi/0.3.9/download",
        type = "tar.gz",
        sha256 = "5c839a674fcd7a98952e593242ea400abe93992746761e38641405d28b00f419",
        strip_prefix = "winapi-0.3.9",
        build_file = Label("//third_party/cargo/remote:BUILD.winapi-0.3.9.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__winapi_build__0_1_1",
        url = "https://crates.io/api/v1/crates/winapi-build/0.1.1/download",
        type = "tar.gz",
        sha256 = "2d315eee3b34aca4797b2da6b13ed88266e6d612562a0c46390af8299fc699bc",
        strip_prefix = "winapi-build-0.1.1",
        build_file = Label("//third_party/cargo/remote:BUILD.winapi-build-0.1.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__winapi_i686_pc_windows_gnu__0_4_0",
        url = "https://crates.io/api/v1/crates/winapi-i686-pc-windows-gnu/0.4.0/download",
        type = "tar.gz",
        sha256 = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6",
        strip_prefix = "winapi-i686-pc-windows-gnu-0.4.0",
        build_file = Label("//third_party/cargo/remote:BUILD.winapi-i686-pc-windows-gnu-0.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__winapi_util__0_1_5",
        url = "https://crates.io/api/v1/crates/winapi-util/0.1.5/download",
        type = "tar.gz",
        sha256 = "70ec6ce85bb158151cae5e5c87f95a8e97d2c0c4b001223f33a334e3ce5de178",
        strip_prefix = "winapi-util-0.1.5",
        build_file = Label("//third_party/cargo/remote:BUILD.winapi-util-0.1.5.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__winapi_x86_64_pc_windows_gnu__0_4_0",
        url = "https://crates.io/api/v1/crates/winapi-x86_64-pc-windows-gnu/0.4.0/download",
        type = "tar.gz",
        sha256 = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f",
        strip_prefix = "winapi-x86_64-pc-windows-gnu-0.4.0",
        build_file = Label("//third_party/cargo/remote:BUILD.winapi-x86_64-pc-windows-gnu-0.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__winit__0_24_0",
        url = "https://crates.io/api/v1/crates/winit/0.24.0/download",
        type = "tar.gz",
        sha256 = "da4eda6fce0eb84bd0a33e3c8794eb902e1033d0a1d5a31bc4f19b1b4bbff597",
        strip_prefix = "winit-0.24.0",
        build_file = Label("//third_party/cargo/remote:BUILD.winit-0.24.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__ws2_32_sys__0_2_1",
        url = "https://crates.io/api/v1/crates/ws2_32-sys/0.2.1/download",
        type = "tar.gz",
        sha256 = "d59cefebd0c892fa2dd6de581e937301d8552cb44489cdff035c6187cb63fa5e",
        strip_prefix = "ws2_32-sys-0.2.1",
        build_file = Label("//third_party/cargo/remote:BUILD.ws2_32-sys-0.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__x11_dl__2_18_5",
        url = "https://crates.io/api/v1/crates/x11-dl/2.18.5/download",
        type = "tar.gz",
        sha256 = "2bf981e3a5b3301209754218f962052d4d9ee97e478f4d26d4a6eced34c1fef8",
        strip_prefix = "x11-dl-2.18.5",
        build_file = Label("//third_party/cargo/remote:BUILD.x11-dl-2.18.5.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__xcursor__0_3_3",
        url = "https://crates.io/api/v1/crates/xcursor/0.3.3/download",
        type = "tar.gz",
        sha256 = "3a9a231574ae78801646617cefd13bfe94be907c0e4fa979cfd8b770aa3c5d08",
        strip_prefix = "xcursor-0.3.3",
        build_file = Label("//third_party/cargo/remote:BUILD.xcursor-0.3.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__xdg__2_2_0",
        url = "https://crates.io/api/v1/crates/xdg/2.2.0/download",
        type = "tar.gz",
        sha256 = "d089681aa106a86fade1b0128fb5daf07d5867a509ab036d99988dec80429a57",
        strip_prefix = "xdg-2.2.0",
        build_file = Label("//third_party/cargo/remote:BUILD.xdg-2.2.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__xml_rs__0_8_4",
        url = "https://crates.io/api/v1/crates/xml-rs/0.8.4/download",
        type = "tar.gz",
        sha256 = "d2d7d3948613f75c98fd9328cfdcc45acc4d360655289d0a7d4ec931392200a3",
        strip_prefix = "xml-rs-0.8.4",
        build_file = Label("//third_party/cargo/remote:BUILD.xml-rs-0.8.4.bazel"),
    )
