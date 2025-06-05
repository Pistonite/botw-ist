use blueflame::env::DlcVer;
use blueflame::game::singleton_instance;
use blueflame::linker;
use blueflame::memory::{MemObject, Ptr};
use blueflame::program;

#[derive(MemObject)]
#[size(0x8)]
struct Pmdm {
    #[offset(0x0)]
    vtable: u64,
}
// make sure the singleton initialization runs without error
#[test]
pub fn test_init_singleton() -> anyhow::Result<()> {
    colog::init();
    let data = std::fs::read("../runtime/program.bfi")?;
    let mut program_bytes = Vec::new();
    let program = program::unpack_zc(&data, &mut program_bytes)?;
    let pmdm_addr_for_test = 0x2222200000;
    let process = linker::init_process(
        program,
        DlcVer::V300,
        0x8888800000,
        0x4000,
        pmdm_addr_for_test,
        20000000,
    )?;

    let pmdm_actual_addr = singleton_instance!(pmdm(process.memory()))?;
    assert_eq!(pmdm_actual_addr.to_raw(), pmdm_addr_for_test);

    let expected_vptr = process.main_start() + 0x02476c38;
    let pmdm_casted: Ptr![Pmdm] = pmdm_actual_addr.reinterpret();
    let actual_vptr = Ptr!(&pmdm_casted->vtable).load(process.memory())?;
    assert_eq!(actual_vptr, expected_vptr);

    Ok(())
}
