# Simple API Benchmark

I've used `wrk` in my MacBook Pro to make some comparative performance tests between several web frameworks.

The test is done with `Hello World` examples and the following `wrk` settings:

`wrk -t12 -c400 -d30s http://localhost:3000`

## JS

* [Express](http://expressjs.com/) [440898]
* [Hapi](https://hapijs.com/) [464458]
* [Koa](https://koajs.com/) [562182]

![results](https://raw.githubusercontent.com/robertohuertasm/simple-api-benchmark/master/js/express-hapi-koa.png 'Results')

## Rust

* [Actix-web](https://actix.rs/) [2523648]
* [Gotham](https://gotham.rs/) [2650183]
* [Thruster](https://github.com/trezm/Thruster) [2945193]

![results](https://raw.githubusercontent.com/robertohuertasm/simple-api-benchmark/master/rust/actix-gotham-thruster.png 'Results')

## .NET Core

* [.NET Core 2.1.300](https://dotnet.github.io/) [1919478]

![results](https://raw.githubusercontent.com/robertohuertasm/simple-api-benchmark/master/net/net.png 'Results')
