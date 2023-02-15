use phper::{functions::Argument, modules::Module, php_get_module, values::ZVal, Error};

enum PhpNumber {
    Double(f64),
    Integer(i64),
}

impl TryFrom<&ZVal> for PhpNumber {
    type Error = Error;

    fn try_from(value: &ZVal) -> Result<Self, Self::Error> {
        match value.expect_double() {
            Ok(float) => Ok(PhpNumber::Double(float)),
            Err(_err) => value
                .expect_long()
                .map(|int| Ok(PhpNumber::Integer(int)))?,
        }
    }
}

impl PhpNumber {
    fn get_float(&self) -> f64 {
        match self {
            Self::Double(num) => *num,
            Self::Integer(num) => *num as f64,
        }
    }
}

fn add(arguments: &mut [ZVal]) -> phper::Result<ZVal> {
    let (x, y) = (&arguments[0], &arguments[1]);

    let x: PhpNumber = x.try_into()?;
    let y: PhpNumber = y.try_into()?;

    match (x, y) {
        (PhpNumber::Integer(x), PhpNumber::Integer(y)) => Ok(ZVal::from(x + y)),
        (x, y) => Ok(ZVal::from(x.get_float() + y.get_float())),
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
