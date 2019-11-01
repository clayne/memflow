// TODO: custom error + result
use std::io::Result;

use byteorder::{BigEndian, ByteOrder, LittleEndian};

use address::{Address, Length};
use arch::{self, Architecture, InstructionSet};

use std::ffi::{CStr, CString};

pub trait PhysicalRead {
    fn phys_read(&mut self, addr: Address, len: Length) -> Result<Vec<u8>>;
}

macro_rules! arch_read_type {
    ($byte_order:expr, $func:ident, $value:expr) => {
        match $byte_order {
            arch::ByteOrder::LittleEndian => LittleEndian::$func($value),
            arch::ByteOrder::BigEndian => BigEndian::$func($value),
        }
    };
}

pub trait VirtualRead {
    fn virt_read(
        &mut self,
        arch: Architecture,
        dtb: Address,
        addr: Address,
        len: Length,
    ) -> Result<Vec<u8>>;

    fn virt_read_addr(
        &mut self,
        arch: Architecture,
        dtb: Address,
        addr: Address,
    ) -> Result<Address> {
        let r = self.virt_read(arch, dtb, addr, arch.instruction_set.len_addr())?;
        Ok(Address::from(arch_read_type!(
            arch.instruction_set.byte_order(),
            read_u64,
            &r
        )))
    }

    fn virt_read_u64(&mut self, arch: Architecture, dtb: Address, addr: Address) -> Result<u64> {
        let r = self.virt_read(arch, dtb, addr, arch.instruction_set.len_u64())?;
        Ok(arch_read_type!(
            arch.instruction_set.byte_order(),
            read_u64,
            &r
        ))
    }

    fn virt_read_u32(&mut self, arch: Architecture, dtb: Address, addr: Address) -> Result<u32> {
        let r = self.virt_read(arch, dtb, addr, arch.instruction_set.len_u32())?;
        Ok(arch_read_type!(
            arch.instruction_set.byte_order(),
            read_u32,
            &r
        ))
    }

    fn virt_read_i64(&mut self, arch: Architecture, dtb: Address, addr: Address) -> Result<i64> {
        let r = self.virt_read(arch, dtb, addr, arch.instruction_set.len_i64())?;
        Ok(arch_read_type!(
            arch.instruction_set.byte_order(),
            read_i64,
            &r
        ))
    }

    fn virt_read_i32(&mut self, arch: Architecture, dtb: Address, addr: Address) -> Result<i32> {
        let r = self.virt_read(arch, dtb, addr, arch.instruction_set.len_i32())?;
        Ok(arch_read_type!(
            arch.instruction_set.byte_order(),
            read_i32,
            &r
        ))
    }

    fn virt_read_f32(&mut self, arch: Architecture, dtb: Address, addr: Address) -> Result<f32> {
        let r = self.virt_read(arch, dtb, addr, arch.instruction_set.len_f32())?;
        Ok(arch_read_type!(
            arch.instruction_set.byte_order(),
            read_f32,
            &r
        ))
    }

    fn virt_read_cstr(&mut self, arch: Architecture, dtb: Address, addr: Address, len: Length) -> Result<String> {
        let mut r = self.virt_read(arch, dtb, addr, len)?;
        match r.iter().enumerate().filter(|(i, c)| **c == 0u8).nth(0) {
            Some((n, _)) => {
                r.truncate(n);
            },
            None => (),
        }

        let v = CString::new(r)?;
        Ok(String::from(v.to_string_lossy()))
    }
}

pub trait PhysicalWrite {
    fn phys_write(&mut self, addr: Address, data: &Vec<u8>) -> Result<Length>;
}

macro_rules! arch_write_type {
    ($byte_order:expr, $func:ident, $buf:expr, $value:expr) => {
        match $byte_order {
            arch::ByteOrder::LittleEndian => LittleEndian::$func($buf, $value),
            arch::ByteOrder::BigEndian => BigEndian::$func($buf, $value),
        }
    };
}

pub trait VirtualWrite {
    fn virt_write(
        &mut self,
        arch: Architecture,
        dtb: Address,
        addr: Address,
        data: &Vec<u8>,
    ) -> Result<Length>;

    fn virt_write_addr(
        &mut self,
        arch: Architecture,
        dtb: Address,
        addr: Address,
        val: Address,
    ) -> Result<Length> {
        let mut buf = vec![0; arch.instruction_set.len_addr().as_usize()];
        arch_write_type!(
            arch.instruction_set.byte_order(),
            write_u64,
            &mut buf,
            val.as_u64()
        );
        self.virt_write(arch, dtb, addr, &buf)
    }

    fn virt_write_u64(
        &mut self,
        arch: Architecture,
        dtb: Address,
        addr: Address,
        val: u64,
    ) -> Result<Length> {
        let mut buf = vec![0; arch.instruction_set.len_u64().as_usize()];
        arch_write_type!(arch.instruction_set.byte_order(), write_u64, &mut buf, val);
        self.virt_write(arch, dtb, addr, &buf)
    }

    fn virt_write_u32(
        &mut self,
        arch: Architecture,
        dtb: Address,
        addr: Address,
        val: u32,
    ) -> Result<Length> {
        let mut buf = vec![0; arch.instruction_set.len_u32().as_usize()];
        arch_write_type!(arch.instruction_set.byte_order(), write_u32, &mut buf, val);
        self.virt_write(arch, dtb, addr, &buf)
    }

    fn virt_write_i64(
        &mut self,
        arch: Architecture,
        dtb: Address,
        addr: Address,
        val: i64,
    ) -> Result<Length> {
        let mut buf = vec![0; arch.instruction_set.len_i64().as_usize()];
        arch_write_type!(arch.instruction_set.byte_order(), write_i64, &mut buf, val);
        self.virt_write(arch, dtb, addr, &buf)
    }

    fn virt_write_i32(
        &mut self,
        arch: Architecture,
        dtb: Address,
        addr: Address,
        val: i32,
    ) -> Result<Length> {
        let mut buf = vec![0; arch.instruction_set.len_i32().as_usize()];
        arch_write_type!(arch.instruction_set.byte_order(), write_i32, &mut buf, val);
        self.virt_write(arch, dtb, addr, &buf)
    }

    fn virt_write_f32(
        &mut self,
        arch: Architecture,
        dtb: Address,
        addr: Address,
        val: f32,
    ) -> Result<Length> {
        let mut buf = vec![0; arch.instruction_set.len_f32().as_usize()];
        arch_write_type!(arch.instruction_set.byte_order(), write_f32, &mut buf, val);
        self.virt_write(arch, dtb, addr, &buf)
    }
}
