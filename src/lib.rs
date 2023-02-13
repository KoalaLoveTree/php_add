// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use phper::{functions::Argument, modules::Module, php_get_module, values::ZVal};
use phper::sys::add_function;

/// The php function, receive arguments with type `ZVal`.
fn add_double(arguments: &mut [ZVal]) -> phper::Result<f64> {
    // Get the first argument, expect the type `ZStr`, and convert to Rust utf-8
    // str.
    let x = arguments[0].expect_double()?;
    let y = arguments[1].expect_double()?;

    Ok(x + y)
}

/// The php function, receive arguments with type `ZVal`.
fn add_int(arguments: &mut [ZVal]) -> phper::Result<i64> {
    // Get the first argument, expect the type `ZStr`, and convert to Rust utf-8
    // str.
    let x = arguments[0].expect_long()?;
    let y = arguments[1].expect_long()?;

    Ok(x + y)
}

/// This is the entry of php extension, the attribute macro `php_get_module`
/// will generate the `extern "C" fn`.
#[php_get_module]
pub fn get_module() -> Module {
    // New `Module` with extension info.
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    // Register function `add`, with one argument `name`.
    module
        .add_function("add_double", add_double)
        .argument(Argument::by_val("x"))
        .argument(Argument::by_val("y"));

    module
        .add_function("add_int", add_int)
        .argument(Argument::by_val("x"))
        .argument(Argument::by_val("y"));

    module

}