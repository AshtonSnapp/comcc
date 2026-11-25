//! Component-C Compiler Targets Translation to LLVM Targets

use inkwell::targets::{
    TargetTriple,
    Target,
};
use inkwell::support::LLVMString;

pub enum TargetArch {
    X86_64,
    Aarch64,
}

pub enum TargetKernel {
    Linux,
    Nt,
    Xnu,
    None,
}

pub enum TargetOs {
    Linux,
    Windows,
    MacOs,
    None,
}

pub enum TargetAbi {
    Gnu,
    Musl,
    Msvc,
    Darwin,
    Uefi,
    None,
}

pub struct TargetQuad {
    arch: TargetArch,
    kernel: TargetKernel,
    os: TargetOs,
    abi: TargetAbi,
}

impl TargetQuad {
    pub fn new(arch: TargetArch, kernel: TargetKernel, os: TargetOs, abi: TargetAbi) -> TargetQuad {
        TargetQuad {
            arch, kernel, os, abi
        }
    }

    pub fn to_llvm(&self) -> Result<Target, LLVMString> {
        let mut triple: &'static str;

        match (self.arch, self.kernel, self.os, self.abi) {
            (TargetArch::X86_64, TargetKernel::Linux, TargetOs::Linux, TargetAbi::Gnu) => triple = "x86_64-unknown-linux-gnu",
            (TargetArch::X86_64, TargetKernel::Linux, TargetOs::Linux, TargetAbi::Musl) => triple = "x86_64-unknown-linux-musl",
            (TargetArch::X86_64, TargetKernel::Nt, TargetOs::Windows, TargetAbi::Msvc) => triple = "x86_64-pc-windows-mscv",
            (TargetArch::X86_64, TargetKernel::Nt, TargetOs::Windows, TargetAbi::Gnu) => triple = "x86_64-w64-windows-gnu",
            (TargetArch::X86_64, TargetKernel::Xnu, TargetOs::MacOs, TargetAbi::Darwin) => triple = "x86_64-apple-macosx",
            (TargetArch::X86_64, TargetKernel::None, TargetOs::None, TargetAbi::Uefi) => triple = "x86_64-unknown-uefi",
            (TargetArch::X86_64, TargetKernel::None, TargetOs::None, TargetAbi::None) => triple = "x86_64-unknown-none-elf",
            (TargetArch::Aarch64, TargetKernel::Linux, TargetOs::Linux, TargetAbi::Gnu) => triple = "aarch64-unkown-linux-gnu",
            (TargetArch::Aarch64, TargetKernel::Linux, TargetOs::Linux, TargetAbi::Musl) => triple = "aarch64-unkown-linux-musl",
            (TargetArch::Aarch64, TargetKernel::Nt, TargetOs::Windows, TargetAbi::Msvc) => triple = "aarch64-pc-windows-mscv",
            (TargetArch::Aarch64, TargetKernel::Nt, TargetOs::Windows, TargetAbi::Gnu) => triple = "aarch64-w64-windows-gnu",
            (TargetArch::Aarch64, TargetKernel::Xnu, TargetOs::MacOs, TargetAbi::Darwin) => triple = "aarch64-apple-macosx",
            (TargetArch::Aarch64, TargetKernel::None, TargetOs::None, TargetAbi::Uefi) => triple = "aarch64-unknown-uefi",
            (TargetArch::Aarch64, TargetKernel::None, TargetOs::None, TargetAbi::None) => triple = "aarch64-unknown-none-elf",
        }

        let triple = TargetTriple::create(triple);

        Target::from_triple(&triple)
    }

    pub fn as_text(&self) -> &'static str {
        match (self.arch, self.kernel, self.os, self.abi) {
            (TargetArch::X86_64, TargetKernel::Linux, TargetOs::Linux, TargetAbi::Gnu) => "x86_64-linux-linux-gnu",
            (TargetArch::X86_64, TargetKernel::Linux, TargetOs::Linux, TargetAbi::Musl) => "x86_64-linux-linux-musl",
            (TargetArch::X86_64, TargetKernel::Nt, TargetOs::Windows, TargetAbi::Msvc) => "x86_64-nt-windows-mscv",
            (TargetArch::X86_64, TargetKernel::Nt, TargetOs::Windows, TargetAbi::Gnu) => "x86_64-nt-windows-gnu",
            (TargetArch::X86_64, TargetKernel::Xnu, TargetOs::MacOs, TargetAbi::Darwin) => "x86_64-xnu-macos-darwin",
            (TargetArch::X86_64, TargetKernel::None, TargetOs::None, TargetAbi::Uefi) => "x86_64-none-none-uefi",
            (TargetArch::X86_64, TargetKernel::None, TargetOs::None, TargetAbi::None) => "x86_64-none-none-none",
            (TargetArch::Aarch64, TargetKernel::Linux, TargetOs::Linux, TargetAbi::Gnu) => "aarch64-linux-linux-gnu",
            (TargetArch::Aarch64, TargetKernel::Linux, TargetOs::Linux, TargetAbi::Musl) => "aarch64-linux-linux-musl",
            (TargetArch::Aarch64, TargetKernel::Nt, TargetOs::Windows, TargetAbi::Msvc) => "aarch64-nt-windows-mscv",
            (TargetArch::Aarch64, TargetKernel::Nt, TargetOs::Windows, TargetAbi::Gnu) => "aarch64-nt-windows-gnu",
            (TargetArch::Aarch64, TargetKernel::Xnu, TargetOs::MacOs, TargetAbi::Darwin) => "aarch64-xnu-macos-darwin",
            (TargetArch::Aarch64, TargetKernel::None, TargetOs::None, TargetAbi::Uefi) => "aarch64-none-none-uefi",
            (TargetArch::Aarch64, TargetKernel::None, TargetOs::None, TargetAbi::None) => "aarch64-none-none-none",
        }
    }
}
