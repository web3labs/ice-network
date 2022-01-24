# Substrate EVM Utilities

This directory is home to a Node.js project with some helpful utilities for working with Substrate
and the EVM pallet.

## Installation and Usage

Use `npm i` to install dependencies. To use these utilities, execute
`node ./utils <command> <parameters>` in the project root (i.e. the parent of this folder).

## Commands

This utility supports the following commands:

### `--substrate-address <address>`

Calculate the substrate address that corresponds to an Ethereum address..

```bash
$ node ./utils --substrate-address 0x6be02d1d3665660d22ff9624b7be0551ee1ac91b
$ 5CNJv1vQjABY9W3BtsV2tzaLCjZepWXaYYzuDGWUUNVvMjcG
```

### `---help`

Print a help message for the utility project.
