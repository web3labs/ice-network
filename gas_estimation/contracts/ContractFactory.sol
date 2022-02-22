// ./contracts/ContractFactory.sol
pragma solidity ^0.8.0;

contract ContractFactory {
  function createInstance() public {
    new Contract();
  }
}

contract Contract {
  constructor() public {}
}

