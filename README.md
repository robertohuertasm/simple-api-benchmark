# Simple API Benchmark

I've used `wrk` in my MacBook Pro to make some comparative performance tests between several web frameworks.

The test is done with `Hello World` examples and the following `wrk` settings:

`wrk -t12 -c400 -d30s http://localhost:3000` -- number of requests in 30 secs

## JS

- [Koa](https://koajs.com/) [603.862]
- [Hapi](https://hapijs.com/) [467.902]
- [Express](http://expressjs.com/) [457.469]

## Rust

- [Hyper](https://hyper.rs) [3.751.419] - 124.974 req/sec
- [Warp](https://github.com/seanmonstar/warp) [3.609.731] - 120.261 req/sec
- [Thruster](https://github.com/trezm/Thruster) [3.125.449] - 103.829 req/sec
- [Tower-web](https://github.com/carllerche/tower-web) [3.112.701] - 103.748 req/sec
- [Gotham](https://gotham.rs/) [3.211.659] - 106.687 req/sec
- [Actix-web](https://actix.rs/) [3.027.716] - 100.577 req/sec
- [Tide](https://github.com/http-rs/tide) [2.963.410] - 98.759 req/sec
- [Rocket](https://rocket.rs/) [16.721] - 555.86 req/sec

## Go

- [Echo](https://github.com/labstack/echo) [2.526.339] - 84.166 req/sec
- [No Framework](https://golang.org/) [2.521.932] - 84.026 req/sec

## .NET Core

- [.NET Core 3.1.101](https://dotnet.github.io/) [1.989.339] - 66.252 req/sec
