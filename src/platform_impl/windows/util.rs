// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::ops::{Deref, DerefMut};

use windows_sys::Win32::UI::WindowsAndMessaging::ACCEL;

pub fn encode_wide<S: AsRef<std::ffi::OsStr>>(string: S) -> Vec<u16> {
    std::os::windows::prelude::OsStrExt::encode_wide(string.as_ref())
        .chain(std::iter::once(0))
        .collect()
}

/// ACCEL wrapper to implement Debug
#[derive(Clone)]
#[repr(transparent)]
pub struct Accel(pub ACCEL);

impl std::fmt::Debug for Accel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ACCEL")
            .field("key", &self.0.key)
            .field("cmd", &self.0.cmd)
            .field("fVirt", &self.0.fVirt)
            .finish()
    }
}

impl Deref for Accel {
    type Target = ACCEL;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Accel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn get_instance_handle() -> windows_sys::Win32::Foundation::HINSTANCE {
    // Gets the instance handle by taking the address of the
    // pseudo-variable created by the microsoft linker:
    // https://devblogs.microsoft.com/oldnewthing/20041025-00/?p=37483

    // This is preferred over GetModuleHandle(NULL) because it also works in DLLs:
    // https://stackoverflow.com/questions/21718027/getmodulehandlenull-vs-hinstance

    extern "C" {
        static __ImageBase: windows_sys::Win32::System::SystemServices::IMAGE_DOS_HEADER;
    }

    unsafe { &__ImageBase as *const _ as _ }
}
