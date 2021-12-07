use fp_evm::LinearCostPrecompile;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
//use pallet_evm_precompile_sha3fips::Sha3FIPS256;
//use fp_evm::Precompile;
use pallet_evm_precompile_sha3fips::Sha3FIPS256;
use pallet_evm_precompile_simple::{ECRecoverPublicKey};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}

#[test]
fn test_sha3_256() -> std::result::Result<(), ()> {
	let input = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
	let expected = b"\
			\xbd\xe3\xf2\x69\x17\x5e\x1d\xcd\xa1\x38\x48\x27\x8a\xa6\x04\x6b\
			\xd6\x43\xce\xa8\x5b\x84\xc8\xb8\xbb\x80\x95\x2e\x70\xb6\xea\xe0\
		";

	let cost: u64 = 1;
    match Sha3FIPS256::execute(input, cost) {
		Ok((_, out)) => {
			assert_eq!(out, expected);
			print!("{:?}", out);
			Ok(())
		}
		Err(e) => {
			panic!("Test not expected to fail: {:?}", e);
		}
	}
}

#[test]
fn test_recover_public_key() -> std::result::Result<(), () > {
	let input = sp_core::bytes::from_hex("0xc5d6c454e4d7a8e8a654f5ef96e8efe41d21a65b171b298925414aa3dc061e3700000000000000000000000000000000000000000000000000000000000000004011de30c04302a2352400df3d1459d6d8799580dceb259f45db1d99243a8d0c64f548b7776cb93e37579b830fc3efce41e12e0958cda9f8c5fcad682c610795").unwrap();
	//Test data is from goloop crypto_test https://github.com/icon-project/goloop/blob/d4569e3c0e5f38489a8400b3e7dc7a7c01da2172/common/crypto/crypto_test.go
	//Note: 0x4 prefix is truncated
	let expected= sp_core::bytes::from_hex("0x48250ebe88d77e0a12bcf530fe6a2cf1ac176945638d309b840d631940c93b78c2bd6d16f227a8877e3f1604cd75b9c5a8ab0cac95174a8a0a0f8ea9e4c10bca").unwrap();
	let cost: u64 = 1;

	match <ECRecoverPublicKey as LinearCostPrecompile>::execute(&input, cost) {
		Ok((_, out)) => {
			assert_eq!(out, expected);
			print!("{:?}", sp_core::bytes::to_hex(out.as_slice(), false));
			Ok(())
		}
		Err(e) => {
			panic!("Test not expected to fail: {:?}", e);
		}
	}
}


#[test]
fn test_verify_recovered_address_from_signature() {
	new_test_ext().execute_with(|| {
		// message with signature
		let signed_data = sp_core::bytes::from_hex("0xc5d6c454e4d7a8e8a654f5ef96e8efe41d21a65b171b298925414aa3dc061e3700000000000000000000000000000000000000000000000000000000000000004011de30c04302a2352400df3d1459d6d8799580dceb259f45db1d99243a8d0c64f548b7776cb93e37579b830fc3efce41e12e0958cda9f8c5fcad682c610795").unwrap();
		// address: hx57b8365292c115d3b72d948272cc4d788fa91f64 => 0x57b8365292c115d3b72d948272cc4d788fa91f64
		let address = sp_core::bytes::from_hex("0x57b8365292c115d3b72d948272cc4d788fa91f64").unwrap();
		//expect the method not to fail when recovering the address
		assert_ok!(TemplateModule::ensure_signed(Origin::signed(1), signed_data, address));
	});
}
