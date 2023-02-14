use crate::ReturnType::{Double, Integer};
use phper::sys::add_function;
use phper::{functions::Argument, modules::Module, php_get_module, values::ZVal};

enum ReturnType {
    Double,
    Integer,
}

fn add(arguments: &mut [ZVal]) -> phper::Result<ZVal> {
    let x = parse_numeric_arg(&mut arguments[0]);
    let y = parse_numeric_arg(&mut arguments[1]);

    let return_type = get_response_type_by_args_type(arguments);

    let result = match return_type {
        Double => ZVal::from(x + y),
        Integer => ZVal::from((x + y) as i64),
    };

    Ok(result)
}

fn parse_numeric_arg(argument: &mut ZVal) -> f64 {
    match argument.expect_double() {
        Ok(float) => float,
        Err(_err) => match argument.expect_long() {
            Ok(int) => int as f64,
            Err(err) => panic!("Input not a number: {err:?}"),
        },
    }
}

fn get_response_type_by_args_type(arguments: &[ZVal]) -> ReturnType {
    let return_type = match arguments[0].expect_double() {
        Ok(_float) => Double,
        Err(_err) => match arguments[0].expect_long() {
            Ok(_int) => Integer,
            Err(err) => panic!("Input not a number: {err:?}"),
        },
    };

    match return_type {
        Double => Double,
        Integer => match arguments[1].expect_double() {
            Ok(_float) => Double,
            Err(_err) => match arguments[1].expect_long() {
                Ok(_int) => Integer,
                Err(err) => panic!("Input not a number: {err:?}"),
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
