// Copyright (c) 2020 Jan Haller. zlib/libpng license.

use deno_core::{ErrBox, JSError};

pub fn expect_error<T>(result: Result<T, ErrBox>, error_type: &str) {
	let err = match result {
		Ok(_) => panic!("Call with {} must not succeed", error_type),
		Err(e) => e
	};

	let err = err.downcast_ref::<JSError>()
		.expect(&format!("{} must lead to JSError type", error_type));

	println!("Expected error occurred:\n{}", err.message);
}
