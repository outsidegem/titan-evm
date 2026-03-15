const { executeRdaPayload } = require('./index.js');

// Minimal RDA Bytecode: PUSH1 0x42 PUSH1 0x00 MSTORE PUSH1 0x20 PUSH1 0x00 RETURN
const payload = Buffer.from("604260005260206000f3", "hex");

console.log("Injecting RDA Payload to Bionic Core...");
console.log(executeRdaPayload(payload));
