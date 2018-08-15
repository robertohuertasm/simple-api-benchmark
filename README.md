# Simple API Benchmark

I've used `wrk` in my MacBook Pro to make some comparative performance tests between several web frameworks.

The test is done with `Hello World` examples and the following `wrk` settings:

`wrk -t12 -c400 -d30s http://localhost:3000` -- number of requests in 30 secs

## JS

- [Koa](https://koajs.com/) [562.182]
- [Hapi](https://hapijs.com/) [464.458]
- [Express](http://expressjs.com/) [440.898]

## Rust

- [Hyper](https://hyper.rs) [3.003.534]
- [Warp](https://github.com/seanmonstar/warp) [3.002.192]
- [Thruster](https://github.com/trezm/Thruster) [2.945.193]
- [Tower-web](https://github.com/carllerche/tower-web) [2.903.518]
- [Actix-web](https://actix.rs/) [2.698.012]
- [Gotham](https://gotham.rs/) [2.650.183]

## .NET Core

- [.NET Core 2.1.302](https://dotnet.github.io/) [1.919.478]
