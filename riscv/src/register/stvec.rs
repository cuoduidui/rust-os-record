//! stvec register

pub use crate::register::mtvec::TrapMode;

/// stvec register
#[derive(Clone, Copy, Debug)]
pub struct Stvec {
    bits: usize,
}

impl Stvec {
    /// Returns the contents of the register as raw bits
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Returns the trap-vector base-address
    pub fn address(&self) -> usize {
        self.bits - (self.bits & 0b11)
    }

    /// Returns the trap-vector mode
    pub fn trap_mode(&self) -> Option<TrapMode> {
        let mode = self.bits & 0b11;
        match mode {
            0 => Some(TrapMode::Direct),
            1 => Some(TrapMode::Vectored),
            _ => None,
        }
    }
}

read_csr_as!(Stvec, 0x105, __read_stvec);
write_csr!(0x105, __write_stvec);
//  //https://www.cnblogs.com/ucas123/p/17629898.html   #[inline]是一个属性（attribute），用于告诉编译器对函数进行内联展开。
// 内联展开是一种编译器优化技术，它将函数的代码直接嵌入到调用处，而不是通过函数调用的方式执行。这样做可以减少函数调用的开销，提高程序的执行效率，但也会增加代码的体积。
/// Writes the CSR 写入csr 
#[inline]
pub unsafe fn write(addr: usize, mode: TrapMode) {
    _write(addr + mode as usize);
}
