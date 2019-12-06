A simple web server that manage a list of domains and periodically check the expiriation dates of their HTTPS certificates, written in Rust.

## Requirements

- Linux Environment (in order to successfully build the sqlite-sys crate)
- Rust/Cargo: 1.39.0+
- NodeJS: 8.16.0+

## Prepare the Web Page

Building the web page first so that you can access it once the web server is launched.

```sh
> cd ./web
> npm i --no-save
> npm run build
```

## Build and Run the Web Server

```
> cargo run
```

You may specific the HTTP port by setting environment variable `PORT` or else default to port 8088.
