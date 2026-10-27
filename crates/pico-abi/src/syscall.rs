pub type RawSyscallArguments = [u32; 4];

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct RawSyscallResult {
    pub status: i32,
    pub value: u32,
}

primitive_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum SyscallError: i32 {
        RegistryNotReady = -1,
        UnknownSyscall = -2,
        NotRunning = -3,
        InvalidArgument = -4,
        InvalidHandle = -5,
        PermissionDenied = -6,
        UnknownError = -0xFF
    }
}
