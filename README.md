[![Build & Test](https://github.com/kumuly/fluttoshi_packages/actions/workflows/build.yml/badge.svg)](https://github.com/kumuly/fluttoshi_packages/actions/workflows/build.yml)

# Fluttoshi packages

We think that all apps that need any kind of global payment mechanism are the best of with Bitcoin on the Lightning Network. We envision a near future where all apps will have a lightweight, non-custodial Lightning wallet embedded, even if they are not necesarily 'Bitcoin' or payment apps at their core.
We want to make it as easy as possible for this to happen. Therefore the packages in this project are designed to be as modular as possible, so that they can be used as building blocks in any Flutter app, regardless of the app's purpose. Also apps that do want to make more full-fledged Bitcoin and Lightning apps might find these packages useful.
We also believe that Flutter is the best framework to build cross-platform apps and that Rust is the best language to build the core functionality of these apps. Therefore we are building these packages in Rust and Flutter.
We mainly use the [Lightning Development Kit](https://www.lightningdevkit.org) and [Bitcoin Development Kit](https://www.bitcoindevkit.org) to have trusted, well-tested and well-maintained Rust libraries to build on.

## Packages

- [x] unified_mnemonic: package to generate and/or restore mnemonics and derive other keys and seeds from them. This makes it possible to have a unified wallet, which means having just one mnemonic for different wallets. For example, have a Bitcoin on-chain wallet, a Bitcoin on-chain wallet with passphrase and a Lightning node wallet, all derived from one 'unified' mnemonic.
- [ ] lightning_message: package to sign and verify Lightning messages. Useful for proving ownership of a node, which in turn is useful for login mechanisms to get rid of username and passwords in apps, amongst other things.
- [ ] ldk: package to set up a Lightning node, based on LDK, in a Flutter app.
- [ ] ldk_bloc: package that uses the ldk package and provides a Bloc to use in Flutter apps.
- [ ] lsp_client: package to connect to a Lightning Service Provider (LSP) and use it to get liquidity, open channels, receive payments offline, etc.
- [ ] payjoin: package to integrate payjoin and its different applications, like payjoin for on-chain transactions and payjoin for opening Lightning channels.
