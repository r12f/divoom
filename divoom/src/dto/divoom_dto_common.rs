macro_rules! impl_divoom_dto_enum_traits {
    ($dto_name:ident, $($enum_value:ident: $enum_string:literal),*) => (
        impl FromStr for $dto_name {
            type Err = String;
            fn from_str(v: &str) -> Result<Self, Self::Err> {
                match v {
                $(
                    $enum_string => Ok($dto_name::$enum_value),
                )*
                    _ => {
                        let parsed = v
                            .parse::<i32>()
                            .map_err(|x| format!("Invalid value for {}: {}", stringify!($dto_name), x))?;
                        Ok($dto_name::Raw(parsed))
                    }
                }
            }
        }

        impl fmt::Display for $dto_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                $(
                    $dto_name::$enum_value => { return write!(f, "{}", $enum_string); },
                )*
                    $dto_name::Raw(n) => { return write!(f, "{}", n); },
                    _ => {}
                }
                panic!("Unsupported value! Please avoid using Raw if possible.");
            }
        }
    )
}

pub(crate) use impl_divoom_dto_enum_traits;
