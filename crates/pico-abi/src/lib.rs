// lib.rs
#![no_std]

pub mod image;
pub mod syscall;

pub mod task;

#[macro_export(local_inner_macros)]
macro_rules! primitive_enum {
    (
        $(#[$attr:meta])*
        $vis:vis enum $name:ident : $repr:ident {
            $($variant:ident = $value:expr),* $(,)?
        }
    ) => {
        $(#[$attr])*
        #[repr($repr)]
        $vis enum $name {
            $($variant = $value),*
        }

        impl core::convert::TryFrom<$repr> for $name {
            type Error = $repr;

            fn try_from(value: $repr) -> core::result::Result<Self, Self::Error> {
                match value {
                    $(
                        n if n == Self::$variant as $repr => Ok(Self::$variant),
                    )*
                    unknown => Err(unknown),
                }
            }
        }
    };
}
