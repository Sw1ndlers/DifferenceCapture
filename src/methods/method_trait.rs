use crate::utils::color::Color;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CalculationMethod {
    pub function: fn(
        threshold: f32,
        distance: f32,
        current_color: &Color,
        previous_color: &Color,
    ) -> Color,
    pub only_compare_first: bool,
    pub name: &'static str,
}

pub trait CalculationMethodTrait {
    fn get_method() -> CalculationMethod;
}

#[macro_export]
macro_rules! create_method {
    ($name:ident, $function:expr, $only_compare_first:expr) => {
        pub struct $name;

        impl CalculationMethodTrait for $name {
            fn get_method() -> CalculationMethod {
                CalculationMethod {
                    function: $function,
                    only_compare_first: $only_compare_first,
                    name: stringify!($name),
                }
            }
        }
    };
}
