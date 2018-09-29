# Simple API Benchmark

I've used `wrk` in my MacBook Pro to make some comparative performance tests between several web frameworks.

The test is done with `Hello World` examples and the following `wrk` settings:

`wrk -t12 -c400 -d30s http://localhost:3000` -- number of requests in 30 secs

## JS

- [Koa](https://koajs.com/) [603.862]
- [Hapi](https://hapijs.com/) [467.902]
- [Express](http://expressjs.com/) [457.469]

## Rust

- [Warp](https://github.com/seanmonstar/warp) [3.037.291]
- [Thruster](https://github.com/trezm/Thruster) [3.028.502]
- [Tower-web](https://github.com/carllerche/tower-web) [3.013.232]
- [Hyper](https://hyper.rs) [3.003.534]
- [Actix-web](https://actix.rs/) [2.698.012]
- [Gotham](https://gotham.rs/) [2.650.183]

## .NET Core

- [.NET Core 2.1.402](https://dotnet.github.io/) [2.076.040]
