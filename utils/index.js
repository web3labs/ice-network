const help =
`--substrate-address  <address>: Calculate the EVM address & substrate address that corresponds to an Ethereum address..
--help: Print this message.`;

if (process.argv.length < 3) {
  console.error('Please provide a command.');
  console.error(help);
  process.exit(9);
}

const command = process.argv[2];
switch (command) {
  case "--substrate-address":
    console.log(require('./substrate-address')());
    break;
  case "--help":
    console.log(help);
    break;
  default:
    console.error(`Unrecognized command: ${command}.`);
    console.error(help);
    process.exit(9);
}
