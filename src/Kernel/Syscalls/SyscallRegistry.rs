use core::cell::UnsafeCell;

use crate::Arch::General::ASM;

use crate::Kernel::Syscalls::Syscall::*;
use crate::Kernel::Syscalls::SyscallTable::*;

const MAX_SYSCALL_TABLES: usize = 8;

pub struct SyscallRegistry {
    tables: [Option<&'static SyscallTable>; MAX_SYSCALL_TABLES],
    sealed: bool
}

impl SyscallRegistry {
    pub const fn new() -> Self {
        Self {
            tables: [None; MAX_SYSCALL_TABLES],
            sealed: false
        }
    }

    pub fn seal(&mut self) {
        self.sealed = true;
    }

    pub fn register(&mut self, table: &'static SyscallTable) {
        if self.sealed {
            panic!("Syscall registry has been sealed");
        }

        assert!((table.id as usize) < MAX_SYSCALL_TABLES, "Syscall table id is out of range");
        
        let slot: &mut Option<&'static SyscallTable> = &mut self.tables[table.id as usize];
        if slot.is_some() {
            panic!("Syscall id {} has been occupied", table.id);
        }
        *slot = Some(table);
    }

    pub fn dispatch(&self, tableId: u8, syscallId: u32, context: SyscallContext, arguments: RawSyscallArguments) -> SyscallResult {
        if !self.sealed {
            return SyscallError::RegistryNotReady.into();
        }

        let Some(table) = self.tables.get(tableId as usize)
                                                    .copied()
                                                    .flatten() 
                                         else {
                                             return SyscallError::UnknownSyscall.into();
                                         };

        let Some(entry) = table.entries.get(syscallId as usize).and_then(Option::as_ref)
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

    pub(crate) unsafe fn initialize (&self, method: impl FnOnce(&mut SyscallRegistry))  {
        ASM::withInterruptsDisabled(|| {
            let registry = unsafe {
                &mut *self.inner.get()
            };

            method(registry);
            registry.seal();
        });
    }

    pub fn dispatch(&self, tableId: u8, syscallId: u32, context: SyscallContext, arguments: RawSyscallArguments) -> SyscallResult {
        let registry = unsafe {
            &*self.inner.get()
        };
        return registry.dispatch(tableId, syscallId, context, arguments);
    }
}

/// Kernel singleton SyscallRegistry wrapper
pub(crate) static SYSCALL_REGISTRY: SyscallRegistryWrapper = SyscallRegistryWrapper::new();