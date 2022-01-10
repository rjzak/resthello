# Welcome to a Profian Coding Exercise!

This repository contains two Rust crates:
  * `server` - a simple HTTP server with a single GET endpoint returning "Hello, World!"
  * `client` - a simple HTTP client that calls the server to get the aforementioned text

You can run the `server` with `cargo run -p server` and the `client` with `cargo run -p client`.

# Feature Requests

This code needs some improvments.

1. Both the `client` and the `server` should be updated to use [structopt] to
   provide CLI documentation.

2. The `client` and `server` should be updated to use versioned APIs. Each
   change below should result in a new API version.

3. The GET endpoint should be converted to `POST` and should receive the name
   of the person to greet. The output should include the person's name rather
   than "World". Likewise, the client needs to be updated to use the `POST`
   endpoint. It should take the name to be sent from a command line argument
   using [structopt].

4. The `POST` endpoint should be converted to receive its input as `JSON`
   rather than text. The client needs to be updated to send `JSON`. Make sure
   that the `HTTP` headers are appropriately updated for this change.

# Exercise Tasks

1. For each of the steps above, file an issue in the repository describing
   (briefly!) the feature that needs to be implemented.

2. Implement each feature as a separate pull request. Make sure to link the
   pull request to the issue so that when the pull request is merged, the
   issue will be closed automatically.

3. Each pull request should contain one or more logical commits with an
   [appropriate commit message].

4. Each pull request (after the first one) should contain an integration test
   for the server.

# Bonus Points

Be prepared to describe (in broad terms), how you'd use OpenID Connect to
require a GitHub login to protect this API from anonymous use.

[structopt]: https://crates.io/crates/structopt
[appropriate commit message]: https://cbea.ms/git-commit/
