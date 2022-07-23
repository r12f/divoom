use rgb::RGB8;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};

#[macro_export]
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
                }
            }
        }

        impl Serialize for $dto_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&format!("{}", self))
            }
        }

        impl<'de> Deserialize<'de> for $dto_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?.to_lowercase();
                let parsed = s.parse::<$dto_name>()
                    .map_err(|_x| de::Error::invalid_value(serde::de::Unexpected::Str(&s), &""))?;
                Ok(parsed)
            }
        }
    )
}

#[macro_export]
macro_rules! impl_divoom_dto_enum_traits_without_raw {
    ($dto_name:ident, $($enum_value:ident: $enum_string:literal),*) => (
        impl FromStr for $dto_name {
            type Err = String;
            fn from_str(v: &str) -> Result<Self, Self::Err> {
                match v {
                $(
                    $enum_string => Ok($dto_name::$enum_value),
                )*
                    _ => Err(format!("Invalid value for {}: {}", stringify!($dto_name), v))
                }
            }
        }

        impl fmt::Display for $dto_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                $(
                    $dto_name::$enum_value => { return write!(f, "{}", $enum_string); },
                )*
                }
            }
        }

        impl Serialize for $dto_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&format!("{}", self))
            }
        }

        impl<'de> Deserialize<'de> for $dto_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?.to_lowercase();
                let parsed = s.parse::<$dto_name>()
                    .map_err(|_x| de::Error::invalid_value(serde::de::Unexpected::Str(&s), &""))?;
                Ok(parsed)
            }
        }
    )
}

pub(crate) use impl_divoom_dto_enum_traits;

pub(crate) fn from_rgb_str<'de, D>(deserializer: D) -> Result<RGB8, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    if s.len() != 7 || s.starts_with('#') {
        return Err(D::Error::invalid_value(
            serde::de::Unexpected::Str(s),
            &"#[0-9A-F]{6}",
        ));
    }

    let v = RGB8::new(
        u8::from_str_radix(&s[0..2], 16).map_err(D::Error::custom)?,
        u8::from_str_radix(&s[2..4], 16).map_err(D::Error::custom)?,
        u8::from_str_radix(&s[4..6], 16).map_err(D::Error::custom)?,
    );

    Ok(v)
}

pub(crate) fn to_rgb_str<S>(v: &RGB8, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let output = format!("#{:02X}{:02X}{:02X}", v.r, v.g, v.b);
    s.serialize_str(&output)
}
