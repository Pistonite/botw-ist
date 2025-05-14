use blueflame::{boot::init_memory, processor::Processor, Core};

fn main() {
    let data = std::fs::read("program.blfm").unwrap();
    let program = blueflame_program::unpack_blueflame(&data).unwrap();
    let (mut mem, mut prox) =
        init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 4150888).unwrap();
    let mut proc = Processor::default();
    let _core = Core {
        mem: &mut mem,
        cpu: &mut proc,
        proxies: &mut prox,
    };
}
