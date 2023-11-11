<!-- PROJECT SHIELDS -->

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]

# PROJECT ARCHIVED

Project has been archived, please see [hedera-sdk-rust](https://github.com/hashgraph/hedera-sdk-rust)

# hedera_rust_client

Rust client for the Hedera Network

Rust client and utils for interacting with the Hedera Network. This library has been written to mimic, where possible, the offical Hedera Java, JS, and Go SDKs.

## Getting Started

You will need a testnet account to run integration tests with the Hedera network
[Hedera TestNet Access](https://docs.hedera.com/guides/testnet/testnet-access)

### Minimum Supported Rust Version

Minimum Supported Rust Version is 1.58.1

### Installation

1. Provide Hedera AccountId and Private Key for TestNet access through `.env` file or through having said env vars set

```sh
ACCOUNT_ID=<INSERT>
PRIVATE_KEY=<INSERT>
```

2. Run all integration tests. NOTE - integration tests w/ Hedera network are ignored by default

```sh
cargo test -- --ignored
```

## Usage

See examples and integration tests

### Tracing

This library uses the [tracing](https://github.com/tokio-rs/tracing) crate for logging during execution

## Roadmap

- [ ] Complete integration testing coverage
- [ ] Update smart contract transactions to latest

See the [open issues](https://github.com/daly4/hedera_rust_client/issues) for a full list of proposed features (and known issues).

## Contributing

Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Run full test suite w/o errors or issues (`cargo test -- --ignored`)
4. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
5. Push to the branch (`git push origin feature/AmazingFeature`)
6. Open a pull request

NOTE - all pull requests must pass all integration tests before merge

## License

Distributed under Apache 2.0 and MIT License. See `LICENSE-MIT` and `LICENSE-APACHE` for more information.

## Contact

Project Link: [https://github.com/daly4/hedera_rust_client](https://github.com/daly4/hedera_rust_client)

## Acknowledgments

- [Hedera SDKs](https://github.com/hashgraph)
- [Tonic](https://github.com/hyperium/tonic)
- [Tokio](https://github.com/tokio-rs/tokio)

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[contributors-shield]: https://img.shields.io/github/contributors/daly4/hedera_rust_client
[contributors-url]: https://github.com/daly4/hedera_rust_client/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/daly4/hedera_rust_client
[forks-url]: https://github.com/daly4/hedera_rust_client/network/members
[stars-shield]: https://img.shields.io/github/stars/daly4/hedera_rust_client
[stars-url]: https://github.com/daly4/hedera_rust_client/stargazers
[issues-shield]: https://img.shields.io/github/issues/daly4/hedera_rust_client
[issues-url]: https://github.com/daly4/hedera_rust_client/issues
