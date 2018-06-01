# Simple API Benchmark

I've used `wrk` in my MacBook Pro to make some comparative performance tests between:

* .NET Core 2.1.101 (C#)
* Actix-web (Rust)
* Express (JS)
* Thruster

The test is done with `Hello World` examples and the following `wrk` settings:

`wrk -t12 -c400 -d30s http://localhost:3000`

![results](https://raw.githubusercontent.com/robertohuertasm/simple-api-benchmark/master/thruster-actix-express-netcore.png 'Results')
