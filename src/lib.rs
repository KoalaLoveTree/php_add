use phper::{functions::Argument, modules::Module, php_get_module, values::ZVal};
use phper::sys::add_function;

fn add_double(arguments: &mut [ZVal]) -> phper::Result<f64> {
    let x = arguments[0].expect_double()?;
    let y = arguments[1].expect_double()?;

    Ok(x + y)
}

fn add_int(arguments: &mut [ZVal]) -> phper::Result<i64> {
    let x = arguments[0].expect_long()?;
    let y = arguments[1].expect_long()?;

    Ok(x + y)
}

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

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