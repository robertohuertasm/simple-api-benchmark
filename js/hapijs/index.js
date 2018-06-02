const Hapi = require('hapi');

const server = Hapi.server({
  host: 'localhost',
  port: 3100,
});

server.route({
  method: 'GET',
  path:'/',
  handler: (request, h) => 'Hello World',
});

async function start() {
  try {
    await server.start();
  } catch (error) {
    console.log(err);
    process.exit(1);
  }
  console.log('Server running at:', server.info.uri);
}

start();