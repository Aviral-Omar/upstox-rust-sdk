# Upstox Rust SDK for API v2

## Introduction

A Rust client for communicating with the <a href="https://upstox.com/uplink/">Upstox API</a>.

Upstox API is a set of rest APIs that provide data required to build a complete investment and trading platform. Execute orders in real time, manage user portfolio, stream live market data (using Websocket), and a lot more with this crate.

- API version: v2

## Environment Variables

These environment variables are used optionally in the SDK depending on the features to be used.

- EMAIL_ID: Email used for Upstox account like "abc@example.com" (Only needed when automating login).
- GOOGLE_AUTHORIZATION_CODE: Authorization code obtained upon Google OAuth 2.0 Authentication which expires in 1 hr. Provide newly fetched value only when manual login page is needed to be skipped (Only needed when automating fetching OTP and using Gmail).
- GOOGLE_CLIENT_ID: Google Client ID for Google Gmail API access (Only needed when automating fetching OTP and using Gmail).
- GOOGLE_CLIENT_SECRET: Google Client Secret for Google Gmail API access (Only needed when automating fetching OTP and using Gmail).
- MOBILE_NUMBER: Mobile number used for Upstox account (Only needed when automating login).
- LOGIN_PIN: Login PIN for Upstox account (Only needed when automating login).
- REDIRECT_PORT: The local port used for redirection for both Upstox API and Gmail API like 8080. Redirect URL provided to both Upstox and Google must be "http://127.0.0.1:$REDIRECT_PORT" if login is needed for authorized endpoint access.
- UPLINK_API_KEY: Upstox API Key. Required for authorized API access ([`Generate Here`](https://account.upstox.com/developer/apps)).
- UPLINK_API_SECRET: Upstox API Secret. Required for authorized API access ([`Generate Here`](https://account.upstox.com/developer/apps)).
- WEBDRIVER_SOCKET: The local socket on which chromedriver or geckodriver is running. They run by default on "http://127.0.0.1:4444" (Only needed when automating login).


## Examples

- [`login-usage`](https://github.com/Aviral-Omar/upstox-rust-sdk/tree/main/examples/login_usage): Example on using login functionality to get access token, automating login, fetching OTP automatically, scheduling automatic re-login.
- [`fetch-instruments`](https://github.com/Aviral-Omar/upstox-rust-sdk/tree/main/examples/fetch_instruments): Example on fetching available instruments on startup and refreshing them daily.
- [`ws-usage`](https://github.com/Aviral-Omar/upstox-rust-sdk/tree/main/examples/ws_usage): Example on using websockets and passing callbacks to handle websocket data and handling app exit when using websockets.

## License

Licensed under <a href="https://choosealicense.com/licenses/mpl-2.0/">MPL 2.0</a>


## Contact

Reach out by mailing me at aviralomar0301@gmail.com
