#[cfg(test)]
mod singleton_tests {
    use blueflame::error::Error;
    use blueflame::memory::{
        traits::{MemRead, MemWrite},
        Reader, Writer,
    };
    use blueflame::{boot::init_memory, memory::traits::Ptr};
    use mem_macro::{MemRead, MemWrite};

    #[derive(MemWrite, MemRead)]
    #[allow(non_snake_case, unused_variables)]
    struct Pmdm {
        #[offset(0x0)]
        __vftable: u64,
        #[offset(0x443c0)]
        mTabsType: [i32; 50],
    }

    #[test]
    pub fn test_init_singleton() {
        let data = std::fs::read("../test_files/program.blfm").unwrap();
        let program = blueflame_program::unpack_blueflame(&data).unwrap();

        //Error in ksys:gdt::Manager::IncreaseLogger::IncreaseLogger at 0x7100dcf8d0
        //Goes to memset_0 and then an error happens somewhere
        //Need to stub memset
        let (mem, _prox) =
            init_memory(&program, 58843136 + 0x100, 0x5000, 0x38a0000, 2000000000).unwrap();
        assert!(true);
        let pmdm_ptr: Ptr<Pmdm> = Ptr::new(0x38a0000);
        let p = Box::new(pmdm_ptr.deref(&mem).unwrap());
        println!("{0:#0x}", p.__vftable);
        println!("{0}", p.mTabsType[0]);
        println!("Done initializing singletons");
        //assert!(false);
    }
}
