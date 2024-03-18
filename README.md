# tower-default-headers-rs
[![Status](https://img.shields.io/badge/status-actively--developed-brightgreen)](https://gitlab.com/jokeyrhyme/tower-default-headers-rs) [![Gitlab pipeline status](https://img.shields.io/gitlab/pipeline-status/jokeyrhyme/tower-default-headers-rs?branch=main)](https://gitlab.com/jokeyrhyme/tower-default-headers-rs/-/pipelines?ref=main) [![Crates.io](https://img.shields.io/crates/v/tower-default-headers)](https://crates.io/crates/tower-default-headers) [![docs.rs](https://img.shields.io/docsrs/tower-default-headers)](https://docs.rs/tower-default-headers)

Tower compatible middleware to set default HTTP response headers.

If you need to set just a small, known set of headers, just use [SetResponseHeaderLayer](https://docs.rs/tower-http/latest/tower_http/set_header/struct.SetResponseHeaderLayer.html) from [tower-http](https://docs.rs/tower-http/latest/tower_http/index.html).

Forked from: https://gitlab.com/jokeyrhyme/tower-default-headers-rs. Updated to be compatible with Axum 0.7 and friends. If you're reading this in the future, see [this issue](https://gitlab.com/jokeyrhyme/tower-default-headers-rs/-/issues/2) to see if the original repo has been updated.

## See also

- tower: [source code](https://github.com/tower-rs/tower) [crate](https://crates.io/crates/tower)

- tokio blog post: [Inventing the `Service` trait](https://tokio.rs/blog/2021-05-14-inventing-the-service-trait)

- owasp-headers-rs: https://github.com/banool/owasp-headers-rs
