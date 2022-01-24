const help = `--substrate-address <address>: Calculate the substrate address that corresponds to an Ethereum address.`;
module.exports = () => {

  const ethereumAddress = process.argv[3];
  
  const crypto = require('@polkadot/util-crypto');

  const addressBytes = Buffer.from(ethereumAddress.slice(2), 'hex');
  const prefixBytes = Buffer.from('evm:');
  const convertBytes = Uint8Array.from(Buffer.concat([ prefixBytes, addressBytes ]));
  const finalAddressHex = crypto.blake2AsHex(convertBytes, 256);//EVM address
  //42+16(in hex format already)= ss58
  
  return crypto.encodeAddress(finalAddressHex, 42);//ss58 substrate address
};


const convertToEvmAddress = (substrateAddress) => {
  const addressBytes = decodeAddress(substrateAddress);
  return '0x' + Buffer.from(addressBytes.subarray(0, 20)).toString('hex');
}