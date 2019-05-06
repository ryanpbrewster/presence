const grpc = require("grpc");
const loader = require("@grpc/proto-loader");

const definition = loader.loadSync("../proto/service.proto");
const proto = grpc.loadPackageDefinition(definition);
const client = new proto.presence.Presence("localhost:50051", grpc.credentials.createInsecure());

async function main() {
  console.log("starting sayHello...");
  await new Promise(resolve => {
    client.sayHello({ name: "Earl" }, (err, resp) => {
      if (err) {
        console.error(err);
      } else {
        console.log(resp);
      }
      resolve();
    });
  });

  console.log("starting put...");
  const my_key = `my_key_${Date.now()}`;
  await new Promise(resolve => {
    client.put({ key: my_key, value: `rnd = ${Math.random()}` }, (err, resp) => {
      if (err) {
        console.error(err);
      } else {
        console.log(resp);
      }
      resolve();
    });
  });

  console.log("starting full scan...");
  let scan = client.scan({});
  await new Promise(resolve => {
    scan.on('data', r => { console.log(r); });
    scan.on('end', () => { resolve(); });
    scan.on('error', e => { console.error(e); resolve(); });
  });
}

main();
