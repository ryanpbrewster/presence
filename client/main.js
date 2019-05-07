const grpc = require("grpc");
const loader = require("@grpc/proto-loader");

const definition = loader.loadSync("../proto/service.proto");
const proto = grpc.loadPackageDefinition(definition);

// This is a GCE f1-micro instance running the presence server
// const HOST = "34.83.221.62";
const HOST = "localhost";
const client = new proto.presence.Presence(`${HOST}:50051`, grpc.credentials.createInsecure());

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

  let park = client.park({});
  await new Promise(resolve => {
    park.on('data', r => { console.log(r); });
    park.on('end', () => { resolve(); });
    park.on('error', e => { console.error(e); resolve(); });
  });
}

main();
