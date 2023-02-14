use crate::ReturnType::{Double, Integer};
use phper::sys::add_function;
use phper::{Error, functions::Argument, modules::Module, php_get_module, values::ZVal};

enum ReturnType {
    Double,
    Integer,
}

fn add(arguments: &mut [ZVal]) -> phper::Result<ZVal> {
    let x = parse_numeric_arg(&mut arguments[0])?;
    let y = parse_numeric_arg(&mut arguments[1])?;

    let return_type = get_response_type_by_args_type(arguments)?;

    match return_type {
        Double => Ok(ZVal::from(x + y)),
        Integer => Ok(ZVal::from((x + y) as i64)),
    }
}

fn parse_numeric_arg(argument: &mut ZVal) -> Result<f64, Error> {
    match argument.expect_double() {
        Ok(float) => Ok(float),
        Err(_err) => match argument.expect_long() {
            Ok(int) => Ok(int as f64),
            Err(err) => Err(err),
        },
    }
}

fn get_response_type_by_args_type(arguments: &[ZVal]) -> Result<ReturnType, Error> {
    let return_type = match arguments[0].expect_double() {
        Ok(_float) => Double,
        Err(_err) => match arguments[0].expect_long() {
            Ok(_int) => Integer,
            Err(err) => return Err(err),
        },
    };

    match return_type {
        Double => Ok(Double),
        Integer => match arguments[1].expect_double() {
            Ok(_float) => Ok(Double),
            Err(_err) => match arguments[1].expect_long() {
                Ok(_int) => Ok(Integer),
                Err(err) => Err(err),
            },
        },
    }
}

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    module
        .add_function("add", add)
        .argument(Argument::by_val("x"))
        .argument(Argument::by_val("y"));

    module
}
