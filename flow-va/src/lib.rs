//#[cfg(feature = "x64")]
pub mod x64;

//#[cfg(feature = "x86_pae")]
pub mod x86_pae;

//#[cfg(feature = "x86")]
pub mod x86;

use std::io::Result;

use arch::{Architecture, InstructionSet};
use address::{Address, Length};
use mem::{PhysicalRead, VirtualRead, PhysicalWrite, VirtualWrite};

// virtual -> physical
pub fn vtop<T: PhysicalRead>(arch: Architecture, mem: &mut T, dtb: Address, addr: Address) -> Result<Address> {
	match arch.instruction_set {
		InstructionSet::X64 => x64::vtop(mem, dtb, addr),
		InstructionSet::X86Pae => x86_pae::vtop(mem, dtb, addr),
		InstructionSet::X86 => x86::vtop(mem, dtb, addr),
	}
}