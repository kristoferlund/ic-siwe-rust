![Sign in with Ethereum for the Internet Computer](/media/header.png)

`ic-siwe` is a project that enables Ethereum wallet-based authentication for applications on the [Internet Computer](https://internetcomputer.org) (ICP). The goal of the project is to enhance the interoperability between Ethereum and ICP, enabling developers to build applications that leverage the strengths of both platforms.

## Features

- **Ethereum Wallet Sign-In**: Enables Ethereum wallet sign-in for ICP applications. Sign in with any eth wallet to generate an ICP identity and session.

- **Session Identity Uniqueness**: Ensures that session identities are specific to each application's context, preventing cross-app identity misuse.

- **Consistent Principal Generation**: Guarantees that logging in with an Ethereum wallet consistently produces the same Principal, irrespective of the client used.

- **Direct Ethereum Address to Principal Mapping**: Creates a one-to-one correlation between Ethereum addresses and Principals within the scope of the current application.

- **Timebound Sessions**: Allows developers to set expiration times for sessions, enhancing security and control.

- **Prebuilt Identity Provider**: Provides a prebuilt canister that can be integrated into any Internet Computer application, independent of the application's programming language.

See this video from the ICP Chain Fusion Hackathon for an overview as well as a deep dive how the SIWE flow works:

[![Integrating with SIWE & SIWS](https://img.youtube.com/vi/lQV4Otp6Y_s/0.jpg)](https://www.youtube.com/watch?v=lQV4Otp6Y_s)

## Usage

Developers have two options to use SIWE in their ICP applications:

1. **Use the prebuilt [ic_siwe_provider](https://github.com/kristoferlund/ic-siwe/tree/main/packages/ic_siwe_provider) canister**: This is the easiest way to integrate SIWE into an Internet Computer application. The pre-built canister is added to the project `dfx.json` and then configured to meet the needs of the application. `ic_siwe_provider` can be added to any ICP application, independent of the application's programming language.

2. **Use the [ic_siwe](https://crates.io/crates/ic_siwe) library**: This allows developers full control over the SIWE integration. The `ic_siwe` Rust library provides all the necessary tools for integrating SIWE into ICP canisters.

### SIWE login flow

The below diagram illustrates the high-level login flow when using the `ic_siwe_provider` canister.

1. An ICP application requests a SIWE message from the `ic_siwe_provider` canister on behalf of the user.

2. The application displays the SIWE message to the user who signs it with their Ethereum wallet.

3. The application sends the signed SIWE message to the `ic_siwe_provider` canister to login the user. The canister verifies the signature and creates an identity for the user.

4. The application retrieves the identity from the `ic_siwe_provider` canister.

5. The application can now use the identity to make authenticated calls to canisters.

![Sign in with Ethereum - Login flow](/media/flow.png)

## Resources

`ic-siwe` consists of two main packages: the Rust support library and the prebuilt identity provider canister. The project also includes demo applications and a JS/TS/React support library for easy frontend integration.

### [ic_siwe](https://github.com/kristoferlund/ic-siwe/tree/main/packages/ic_siwe)

Rust library that provides the necessary tools for integrating Sign-In with Ethereum (SIWE) into ICP canisters, allowing users to sign in using their Ethereum wallets.

### [ic-siwe-provider](https://github.com/kristoferlund/ic-siwe/tree/main/packages/ic_siwe_provider)

Prebuilt canister serving as a SIWE identity provider for Internet Computer canisters. `ic_siwe-provider` packages the [ic_siwe](https://github.com/kristoferlund/ic-siwe/tree/main/packages/ic_siwe) library and makes it available as a canister that can easily be integrated into any Internet Computer application, independent of the application's programming language.

### [ic-siwe-js](https://www.npmjs.com/package/ic-siwe-js)

JS/TS support library. Includes a React hook and context provider for easy frontend integration.

### [ic-use-actor](https://github.com/kristoferlund/ic-use-actor)

React hook and context provider for managing Internet Computer (ICP) actors with features like type safety and request/response interceptors. `ic-use-actor` makes interacting with Internet Computer canisters more fun!

## Demos

The project includes demo applications that demonstrate how to integrate SIWE into Internet Computer canisters using the [ic-siwe-js](https://www.npmjs.com/package/ic-siwe-js) support library and [ic-siwe-provider](https://github.com/kristoferlund/ic-siwe/tree/main/packages/ic_siwe_provider) canister.

### Vanilla TypeScript Demo

GitHub: [ic-siwe-vanilla-ts-demo](https://github.com/kristoferlund/ic-siwe-vanilla-ts-demo)
Live demo: [https://lglxt-uyaaa-aaaal-qsgbq-cai.icp0.io](https://lglxt-uyaaa-aaaal-qsgbq-cai.icp0.io)

### React Demo

GitHub: [ic-siwe-react-demo](https://github.com/kristoferlund/ic-siwe-react-demo-rust)
Live demo: [https://shtr2-2iaaa-aaaal-qckva-cai.icp0.io](https://shtr2-2iaaa-aaaal-qckva-cai.icp0.io)

### Vue Demo

GitHub: [ic-siwe-vue-demo](https://github.com/kristoferlund/ic-siwe-vue-demo)
Live demo: [https://kmevj-wiaaa-aaaal-qsggq-cai.icp0.io](https://kmevj-wiaaa-aaaal-qsggq-cai.icp0.io)

## Updates

See the respective package CHANGELOG for details on updates.

## Contributing

Contributions are welcome. Please submit your pull requests or open issues to propose changes or report bugs.

## Author

- [kristofer@kristoferlund.se](mailto:kristofer@kristoferlund.se)
- Twitter: [@kristoferlund](https://twitter.com/kristoferlund)
- Discord: kristoferkristofer
- Telegram: [@kristoferkristofer](https://t.me/kristoferkristofer)

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

## Future Plans

The project is still in active development. Before using `ic-siwe` in production, I would like to do a more formal security audit.

Also, I want to integrate SIWE into more demo applications, ideally some wallet application.

Most likely, there are features missing in the current implementation. If you have any ideas or requests for features, please let me know by [opening an issue](https://github.com/kristoferlund/ic-siwe/issues).
