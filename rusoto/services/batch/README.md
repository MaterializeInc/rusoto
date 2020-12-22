
# Rusoto Batch
Rust SDK for AWS Batch

⚠️ **This is the [Materialize](https://materialize.com) fork of Rusoto.** ⚠️

Rusoto has been [unmaintained for several months](https://github.com/rusoto/rusoto/issues/1651).
We expect that Amazon will soon announce plans to take over Rusoto or release
an official Rust SDK. In the meantime, we are performing a minimal amount of
maintenance. We will accept dependency bumps and obvious bug fixes.

Crates are published with an "mz" prefix, as in `mz_rusoto_core`.

---

You may be looking for:

* [An overview of Rusoto][rusoto-overview]
* [AWS services supported by Rusoto][supported-aws-services]
* [API documentation][api-documentation]
* [Getting help with Rusoto][rusoto-help]

## Requirements

Rust stable or beta are required to use Rusoto. Nightly is tested, but not guaranteed to be supported. Older
versions _may_ be supported. The currently supported Rust versions can be found in the Rusoto project
[`travis.yml`](https://github.com/rusoto/rusoto/blob/master/.travis.yml).

On Linux, OpenSSL is required.

## Installation

To use `mz_rusoto_batch` in your application, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
mz_rusoto_batch = "0.46.0"
```

## Crate Features
- `native-tls` - use platform-specific TLS implementation.
- `rustls` - use rustls TLS implementation.
- `serialize_structs` - output structs of most operations get `derive(Serialize)`.
- `deserialize_structs` - input structs of most operations get `derive(Deserialize)`.

Note: the crate will use the `native-tls` TLS implementation by default.

## Contributing

See [CONTRIBUTING][contributing].

## License

Rusoto is distributed under the terms of the MIT license.

See [LICENSE][license] for details.

[api-documentation]: https://docs.rs/mz_rusoto_batch "API documentation"
[license]: https://github.com/rusoto/rusoto/blob/master/LICENSE "MIT License"
[contributing]: https://github.com/rusoto/rusoto/blob/master/CONTRIBUTING.md "Contributing Guide"
[rusoto-help]: https://www.rusoto.org/help.html "Getting help with Rusoto"
[rusoto-overview]: https://www.rusoto.org/ "Rusoto overview"
[supported-aws-services]: https://www.rusoto.org/supported-aws-services.html "List of AWS services supported by Rusoto"
        
