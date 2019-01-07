# Simple API Benchmark

I've used `wrk` in my MacBook Pro to make some comparative performance tests between several web frameworks.

The test is done with `Hello World` examples and the following `wrk` settings:

`wrk -t12 -c400 -d30s http://localhost:3000` -- number of requests in 30 secs

## JS

- [Koa](https://koajs.com/) [603.862]
- [Hapi](https://hapijs.com/) [467.902]
- [Express](http://expressjs.com/) [457.469]

## Rust

- [Thruster](https://github.com/trezm/Thruster) [2.927.239]
- [Hyper](https://hyper.rs) [2.875.151]
- [Tower-web](https://github.com/carllerche/tower-web) [2.858.096]
- [Warp](https://github.com/seanmonstar/warp) [2.866.438]
- [Gotham](https://gotham.rs/) [2.840.204]
- [Actix-web](https://actix.rs/) [2.816.621]
- [Rocket](https://rocket.rs/) [16.891]

## .NET Core

- [.NET Core 2.2.101](https://dotnet.github.io/) [1.987.003]
