// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use super::{SECP256K1, Public, Secret, Error};
use secp256k1::key;
use secp256k1::constants::{GENERATOR_X, GENERATOR_Y};

/// Inplace multiply public key by secret key (EC point * scalar)
pub fn public_mul_secret(public: &mut Public, secret: &Secret) -> Result<(), Error> {
	let key_secret = secret.to_secp256k1_secret()?;
	let mut key_public = to_secp256k1_public(public)?;
	key_public.mul_assign(&SECP256K1, &key_secret)?;
	set_public(public, &key_public);
	Ok(())
}

/// Inplace add one public key to another (EC point + EC point)
pub fn public_add(public: &mut Public, other: &Public) -> Result<(), Error> {
	let mut key_public = to_secp256k1_public(public)?;
	let other_public = to_secp256k1_public(other)?;
	key_public.add_assign(&SECP256K1, &other_public)?;
	set_public(public, &key_public);
	Ok(())
}

/// Return base point of secp256k1
pub fn generation_point() -> Public {
	let mut public_sec_raw = [0u8; 65];
	public_sec_raw[0] = 4;
	public_sec_raw[1..33].copy_from_slice(&GENERATOR_X);
	public_sec_raw[33..65].copy_from_slice(&GENERATOR_Y);

	let public_key = key::PublicKey::from_slice(&SECP256K1, &public_sec_raw)
		.expect("constructing using predefined constants; qed");
	let mut public = Public::default();
	set_public(&mut public, &public_key);
	public
}

fn to_secp256k1_public(public: &Public) -> Result<key::PublicKey, Error> {
	let public_data = {
		let mut temp = [4u8; 65];
		(&mut temp[1..65]).copy_from_slice(&public[0..64]);
		temp
	};

	Ok(key::PublicKey::from_slice(&SECP256K1, &public_data)?)
}

fn set_public(public: &mut Public, key_public: &key::PublicKey) {
	let key_public_serialized = key_public.serialize_vec(&SECP256K1, false);
	public.copy_from_slice(&key_public_serialized[1..65]);
}
