mod cpu;
mod memory;

use cpu::CPU;
use memory::Memory;

use goblin::Object;
use memmap::MmapOptions;
use rangemap::RangeMap;
use std::fs::File;
use std::path::PathBuf;
use std::result;
use structopt::StructOpt;

#[derive(Debug)]
pub enum Error {
    ObjectFormatNotSupported(String),
    ProgramLoadError(String),
    ProgramParseError(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::ProgramLoadError(e.to_string())
    }
}

impl From<goblin::error::Error> for Error {
    fn from(e: goblin::error::Error) -> Self {
        Error::ProgramParseError(e.to_string())
    }
}

type Result<T> = result::Result<T, Error>;

#[derive(Debug, StructOpt)]
#[structopt(name = "riscv-rs", about = "A RISC-V CPU emulator, written in Rust.")]
struct Opt {
    /// FIlename of an ELF executable to run in the emulator.
    #[structopt(parse(from_os_str), required = true)]
    filename: PathBuf,
}

fn run(filename: &PathBuf) -> Result<()> {
    let mut cpu = load(filename)?;
    while !cpu.halted() {
        cpu.exec();
    }
    Ok(())
}

fn load(filename: &PathBuf) -> Result<CPU> {
    let file = File::open(filename)?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };
    let object = Object::parse(&mmap)?;
    let (mapping, pc) = match object {
        Object::Elf(elf) => {
            let mut mapping = RangeMap::new();
            for ph in elf.program_headers {
                let start = ph.p_vaddr;
                let end = ph.p_vaddr + ph.p_memsz;
                let offset = ph.p_offset;
                mapping.insert(start..end, (start, offset));
            }
            let pc = elf.header.e_entry as u32;
            (mapping, pc)
        }
        Object::PE(_) => return Err(Error::ObjectFormatNotSupported(String::from("PE"))),
        Object::Mach(_) => return Err(Error::ObjectFormatNotSupported(String::from("Mach-O"))),
        Object::Archive(_) => {
            return Err(Error::ObjectFormatNotSupported(String::from("Unix a.out")))
        }
        Object::Unknown(magic) => {
            return Err(Error::ObjectFormatNotSupported(String::from(format!(
                "unknown magic: {:#x}",
                magic
            ))))
        }
    };
    let mem = Memory::new(mapping, mmap);
    let cpu = CPU::new(mem, pc);
    Ok(cpu)
}

fn main() {
    let opts = Opt::from_args();
    std::process::exit(match run(&opts.filename) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}
