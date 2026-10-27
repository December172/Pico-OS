#[doc(hidden)]
#[macro_export]
macro_rules! task_trampoline {
    (
        $(#[$attr:meta])*
        $_vis:vis fn $name:ident($argument:ident: *mut ()) $body:block
    ) => {
        $(#[$attr])*
        #[unsafe(no_mangle)]  // for entry address in linker script
        pub extern "C" fn $name(argument: *mut ()) -> ! {
            fn task_body($argument: *mut()) $body

            task_body(argument);

            $crate::task::exit();

            ::core::unreachable!();
        }
    };

    ($($other:tt)*) => {
        ::core::compile_error!(
            "expected fn name(argument: *mut ()) { ... }"
        );
    };
}
