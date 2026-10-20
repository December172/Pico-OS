use core::cell::UnsafeCell;

use crate::arch::general::asm;

use crate::kernel::syscalls::syscall::*;
use crate::kernel::syscalls::syscall_table::*;

use crate::util::syscall::SyscallArguments;

const MAX_SYSCALL_TABLES: usize = 8;

pub struct SyscallRegistry {
    tables: [Option<&'static SyscallTable>; MAX_SYSCALL_TABLES],
    sealed: bool,
}

impl SyscallRegistry {
    pub const fn new() -> Self {
        Self {
            tables: [None; MAX_SYSCALL_TABLES],
            sealed: false,
        }
    }

    pub fn seal(&mut self) {
        self.sealed = true;
    }

    pub fn register(&mut self, table: &'static SyscallTable) {
        if self.sealed {
            panic!("syscall registry has been sealed");
        }

        assert_ne!(table.id, 0, "Table id #0 is reserved");
        assert!(
            (table.id as usize) < MAX_SYSCALL_TABLES,
            "syscall table id is out of range"
        );

        let slot: &mut Option<&'static SyscallTable> = &mut self.tables[table.id as usize];
        if slot.is_some() {
            panic!("syscall id {} has been occupied", table.id);
        }
        *slot = Some(table);
    }

    pub fn dispatch(
        &self,
        table_id: u8,
        syscall_id: u32,
        context: SyscallContext,
        arguments: SyscallArguments,
    ) -> SyscallDispatchResult {
        if !self.sealed {
            return SyscallError::RegistryNotReady.into();
        }

        let Some(table) = self.tables.get(table_id as usize).copied().flatten() else {
            return SyscallError::UnknownSyscall.into();
        };

        let Some(entry) = table
            .entries
            .get(syscall_id as usize)
            .and_then(Option::as_ref)
        else {
            return SyscallError::UnknownSyscall.into();
        };

        return entry.invoke(context, arguments);
    }
}

pub(crate) struct SyscallRegistryWrapper {
    inner: UnsafeCell<SyscallRegistry>,
}

unsafe impl Sync for SyscallRegistryWrapper {}

impl SyscallRegistryWrapper {
    pub const fn new() -> Self {
        Self {
            inner: UnsafeCell::new(SyscallRegistry::new()),
        }
    }

    pub(crate) unsafe fn initialize(&self, method: impl FnOnce(&mut SyscallRegistry)) {
        asm::with_interrupts_disabled(|| {
            let registry = unsafe { &mut *self.inner.get() };

            method(registry);
            registry.seal();
        });
    }

    pub fn dispatch(
        &self,
        table_id: u8,
        syscall_id: u32,
        context: SyscallContext,
        arguments: SyscallArguments,
    ) -> SyscallDispatchResult {
        let registry = unsafe { &*self.inner.get() };
        return registry.dispatch(table_id, syscall_id, context, arguments);
    }
}

/// kernel singleton SyscallRegistry wrapper
pub(crate) static SYSCALL_REGISTRY: SyscallRegistryWrapper = SyscallRegistryWrapper::new();
