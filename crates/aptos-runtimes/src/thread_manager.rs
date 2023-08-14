// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::spawn_named_runtime;
use libc::{cpu_set_t, sched_getaffinity, sched_setaffinity, CPU_ISSET, CPU_SET, CPU_SETSIZE};
use once_cell::sync::Lazy;
use tokio::runtime::{Builder, Handle, Runtime};

pub static THREAD_MANAGER: Lazy<ThreadManager> = Lazy::new(|| ThreadManager::new());

pub struct ThreadManager {
    vm_runtime: Runtime,
}

impl ThreadManager {
    pub fn get_vm_execution_runtime(&self) -> Handle {
        self.vm_runtime.handle().clone()
    }

    fn new() -> Self {
        let core_ids = core_affinity::get_core_ids().unwrap();
        assert!(core_ids.len() > 48);
        let mut vm_cpu_set = Self::new_cpu_set();
        let mut other_cpu_set = Self::new_cpu_set();
        for core_id in core_ids.iter().take(32) {
            unsafe { CPU_SET(core_id.id, &mut vm_cpu_set) };
        }
        for core_id in core_ids.iter().skip(32) {
            unsafe { CPU_SET(core_id.id, &mut other_cpu_set) };
        }

        let vm_runtime = spawn_named_runtime("vm_exe".into(), Some(64), move || {
            unsafe {
                sched_setaffinity(
                    0, // Defaults to current thread
                    std::mem::size_of::<cpu_set_t>(),
                    &vm_cpu_set,
                );
            };
        });

        Self { vm_runtime }
    }

    fn new_cpu_set() -> cpu_set_t {
        unsafe { std::mem::zeroed::<cpu_set_t>() }
    }
}
