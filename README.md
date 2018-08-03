# Simple API Benchmark

I've used `wrk` in my MacBook Pro to make some comparative performance tests between several web frameworks.

The test is done with `Hello World` examples and the following `wrk` settings:

`wrk -t12 -c400 -d30s http://localhost:3000` -- number of requests in 30 secs

## JS

- [Express](http://expressjs.com/) [440.898]
- [Hapi](https://hapijs.com/) [464.458]
- [Koa](https://koajs.com/) [562.182]
- [results](https://raw.githubusercontent.com/robertohuertasm/simple-api-benchmark/master/js/express-hapi-koa.png)

## Rust

- [Actix-web](https://actix.rs/) [2.523.648]
- [Gotham](https://gotham.rs/) [2.650.183]
- [Thruster](https://github.com/trezm/Thruster) [2.945.193]
- [Warp](https://github.com/seanmonstar/warp) [3.002.192]
- [Hyper](https://hyper.rs) [3.003.534]
- [results](https://raw.githubusercontent.com/robertohuertasm/simple-api-benchmark/master/rust/actix-gotham-thruster-warp-hyper.png)

## .NET Core

- [.NET Core 2.1.300](https://dotnet.github.io/) [1.919.478]
- [results](https://raw.githubusercontent.com/robertohuertasm/simple-api-benchmark/master/net/net.png)
