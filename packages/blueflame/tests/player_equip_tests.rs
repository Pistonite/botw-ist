#[cfg(test)]
mod player_equipment_tests {
    use blueflame::{boot::init_memory, processor::Processor, Core};

    #[test]
    pub fn test_create_player_equipment() {
        let data = std::fs::read("./test_files/program.blfm").unwrap();
        let program = blueflame_program::unpack_blueflame(&data).unwrap();
        let (mut mem, mut prox) =
            init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 20000000).unwrap();
        let mut cpu = Processor::default();
        let mut core = Core {
            cpu: &mut cpu,
            mem: &mut mem,
            proxies: &mut prox,
        };
        core.setup().unwrap();

        core.create_player_equipment().unwrap();
    }
}
