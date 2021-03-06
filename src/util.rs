// Copyright (c) 2020 Jan Haller. zlib/libpng license.

use deno_core::ErrBox;

use crate::{JsValue, Script};

/// Evaluates a standalone Javascript expression, and returns the result as a JSON value.
///
/// If there is an error, Err will be returned.
/// This function is primarily useful for small standalone experiments. Usually, you would want to use the [`Script`](struct.Script.html) struct
/// for more sophisticated Rust->JS interaction.
pub fn eval_json(js_expr: &str) -> Result<JsValue, ErrBox> {
	let code = format!("
		function __rust_expr() {{
			return ({expr});
		}}
	", expr = js_expr);

	let mut script = Script::from_string(&code)?;
	script.call_json("__rust_expr", &JsValue::Null)
}