use phper::{Error, functions::Argument, modules::Module, php_get_module, values::ZVal};
use phper::sys::add_function;

const ADD_RETURN_TYPE_INT: &str = "ADD_RETURN_TYPE_INT";
const ADD_RETURN_TYPE_DOUBLE: &str = "ADD_RETURN_TYPE_DOUBLE";

fn add(arguments: &mut [ZVal]) -> phper::Result<ZVal> {
    let x = parse_numeric_arg(&mut arguments[0]);
    let y = parse_numeric_arg(&mut arguments[1]);

    let return_type = parse_return_type_arg(arguments)?;

    let result;

    if return_type == ADD_RETURN_TYPE_DOUBLE {
        result = ZVal::from(x + y)
    } else {
        result = ZVal::from((x + y).round() as i64);
    }

    Ok(result)
}

fn parse_numeric_arg(argument: &mut ZVal) -> f64 {
    let number = match argument.expect_double() {
        Ok(float) => float,
        Err(_err) => {
            match argument.expect_long() {
                Ok(int) => {
                    int as f64
                }
                Err(err) => panic!("Input not a number: {:?}", err),
            }
        }
    };

    number
}

fn parse_return_type_arg(arguments: &mut [ZVal]) -> Result<&str, Error> {
    let return_type: &str;

    if arguments.get(2).is_none() {
        return_type = ADD_RETURN_TYPE_DOUBLE
    } else {
        return_type = arguments[2].expect_z_str()?.to_str()?;
    }

    match return_type {
        ADD_RETURN_TYPE_INT => Ok(ADD_RETURN_TYPE_INT),
        _ => Ok(ADD_RETURN_TYPE_DOUBLE)
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
        .argument(Argument::by_val("y"))
        .argument(Argument::by_val_optional("return_type"));

    module
        .add_constant("ADD_RETURN_TYPE_INT", ADD_RETURN_TYPE_INT);

    module
        .add_constant("ADD_RETURN_TYPE_DOUBLE", ADD_RETURN_TYPE_DOUBLE);

    module
}