use crate::dto::*;

macro_rules! impl_divoom_dto_enum_to_pixoo_contract_converter {
    ($dto_name:ident, $($enum_value:ident: $enum_number:literal),*) => (
        impl From<i32> for $dto_name {
            fn from(v: i32) -> Self {
                match v {
                $(
                    $enum_number => $dto_name::$enum_value,
                )*
                    _ => $dto_name::Raw(v),
                }
            }
        }

        impl From<$dto_name> for i32 {
            fn from(v: $dto_name) -> Self {
                match v {
                $(
                    $dto_name::$enum_value => $enum_number,
                )*
                    $dto_name::Raw(v) => v,
                }
            }
        }
    )
}

impl_divoom_dto_enum_to_pixoo_contract_converter!(DivoomChannelType, Clock: 0, CloudChannel: 1, Visualizer: 2, CustomPage: 3);
impl_divoom_dto_enum_to_pixoo_contract_converter!(DivoomCloudChannelType, Gallery: 0, Fav: 1, Artist: 2);
impl_divoom_dto_enum_to_pixoo_contract_converter!(DivoomDeviceHighLightMode, Off: 0, On: 1);
impl_divoom_dto_enum_to_pixoo_contract_converter!(DivoomDeviceHourMode, Hour12: 0, Hour24: 1);
impl_divoom_dto_enum_to_pixoo_contract_converter!(DivoomDeviceMirrorMode, Off: 0, On: 1);
impl_divoom_dto_enum_to_pixoo_contract_converter!(DivoomDeviceTemperatureUnit, Celsius: 0, Fahrenheit: 1);
impl_divoom_dto_enum_to_pixoo_contract_converter!(DivoomDeviceRotationAngle, None: 0, Rotate90: 1, Rotate180: 2, Rotate270: 3);
impl_divoom_dto_enum_to_pixoo_contract_converter!(DivoomDeviceScreenPowerState, Off: 0, On: 1);
impl_divoom_dto_enum_to_pixoo_contract_converter!(DivoomFileAnimationSourceType, LocalFile: 0, LocalFolder: 1, Url: 2);
impl_divoom_dto_enum_to_pixoo_contract_converter!(DivoomFontType, Scrollable: 0, NotScrollable: 1);
impl_divoom_dto_enum_to_pixoo_contract_converter!(DivoomTextAnimationScrollDirection, Left: 0, Right: 1);
impl_divoom_dto_enum_to_pixoo_contract_converter!(DivoomTextAnimationAlign, Left: 1, Middle: 2, Right: 3); // Yes, this starts from 1 not 0.
impl_divoom_dto_enum_to_pixoo_contract_converter!(DivoomToolCountdownAction, Stop: 0, Start: 1);
impl_divoom_dto_enum_to_pixoo_contract_converter!(DivoomToolNoiseAction, Stop: 0, Start: 1);
impl_divoom_dto_enum_to_pixoo_contract_converter!(DivoomToolStopwatchAction, Stop: 0, Start: 1, Reset: 2);
