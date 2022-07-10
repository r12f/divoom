macro_rules! impl_divoom_dto_enum_traits {
    ($dto_name:ty, $($enum_value:ident: $enum_string:literal),*) => (
        impl FromStr for $dto_name {
            type Err = String;
            fn from_str(v: &str) -> Result<Self, Self::Err> {
                match v {
                $(
                    $enum_string => Ok(<$dto_name>::$enum_value),
                )*
                    _ => {
                        let parsed = v
                            .parse::<i32>()
                            .map_err(|x| format!("Invalid value for {}: {}", stringify!($dto_name), x))?;
                        Ok(<$dto_name>::Raw(parsed))
                    }
                }
            }
        }

        impl fmt::Display for $dto_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                $(
                    <$dto_name>::$enum_value => { return write!(f, "{}", $enum_string); },
                )*
                    // error[E0658]: usage of qualified paths in this context is experimental
                    // <$dto_name>::Raw(n) => write!(f, "{}", n),
                    _ => {}
                }

                // error[E0658]: usage of qualified paths in this context is experimental
                // if let <$dto_name>::Raw(n) = self {
                // // Or if let <$dto_name>::Raw(_) = self {
                //     write!(f, "{}", n)
                // }
                // :(

                panic!("Unsupported value! Please avoid using Raw if possible.");
            }
        }
    )
}

pub(crate) use impl_divoom_dto_enum_traits;
