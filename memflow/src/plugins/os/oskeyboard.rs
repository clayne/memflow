pub mod keyboard;
pub use keyboard::{ArcPluginKeyboard, PluginKeyboard};

pub mod keyboardstate;
pub use keyboardstate::ArcPluginKeyboardState;

use crate::error::*;

use crate::os::{OSKeyboardInner, Keyboard};
use std::ffi::c_void;

use super::super::COptArc;
use super::PluginOSKeyboard;
use super::{MUArcPluginKeyboard, MUPluginKeyboard};

use libloading::Library;

pub type OpaqueOSKeyboardFunctionTable = OSKeyboardFunctionTable<'static, c_void, c_void>;

impl Copy for OpaqueOSKeyboardFunctionTable {}

impl Clone for OpaqueOSKeyboardFunctionTable {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
pub struct OSKeyboardFunctionTable<'a, P, T> {
    pub keyboard: extern "C" fn(os: &'a mut T, lib: COptArc<Library>, out: &mut MUPluginKeyboard<'a>) -> i32,
    pub into_keyboard:
        extern "C" fn(os: &mut T, lib: COptArc<Library>, out: &mut MUArcPluginKeyboard) -> i32,
    phantom: std::marker::PhantomData<P>,
}

impl<'a, P: 'static + Keyboard + Clone, T: PluginOSKeyboard<P>> Default
    for &'a OSKeyboardFunctionTable<'a, P, T>
{
    fn default() -> Self {
        &OSKeyboardFunctionTable {
            keyboard: c_keyboard,
            into_keyboard: c_into_keyboard,
            phantom: std::marker::PhantomData {},
        }
    }
}

impl<'a, P: 'static + Keyboard + Clone, T: PluginOSKeyboard<P>> OSKeyboardFunctionTable<'a, P, T> {
    pub fn as_opaque(&self) -> &OpaqueOSKeyboardFunctionTable {
        unsafe { &*(self as *const Self as *const OpaqueOSKeyboardFunctionTable) }
    }
}

extern "C" fn c_keyboard<'a, T: 'a + OSKeyboardInner<'a>>(
    os: &'a mut T,
    lib: COptArc<Library>,
    out: &mut MUPluginKeyboard<'a>,
) -> i32 {
    os.keyboard()
        .map(|k| PluginKeyboard::new(k, lib))
        .as_int_out_result(out)
}

extern "C" fn c_into_keyboard<P: 'static + Keyboard + Clone, T: 'static + PluginOSKeyboard<P>>(
    os: &mut T,
    lib: COptArc<Library>,
    out: &mut MUArcPluginKeyboard,
) -> i32 {
    let os = unsafe { Box::from_raw(os) };
    os.into_keyboard()
        .map(|k| ArcPluginKeyboard::new(k, lib))
        .as_int_out_result(out)
}
