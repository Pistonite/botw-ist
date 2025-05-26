use blueflame::env::{Environment, DlcVer};
use blueflame::memory::{MemObject, Ptr};
use blueflame::game::singleton_instance;
use blueflame::linker;
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
    let data = std::fs::read("./test_files/program.blfm")?;
    let program = program::unpack(&data)?;

    let env = Environment::new(program.ver, DlcVer::V300);
    let pmdm_addr_for_test = 0x38a0000;
    let process = linker::init_process(
        &program,
        env.dlc_ver,
        0x8888800000,
        0x4000,
        pmdm_addr_for_test,
        2000000
    )?;

    let pmdm_actual_addr = singleton_instance!(pmdm(process.memory()))?;
    assert_eq!(pmdm_actual_addr.to_raw(), pmdm_addr_for_test);

    let expected_vptr = process.main_start() + 0x02476c38;
    let pmdm_casted: Ptr![Pmdm] = pmdm_actual_addr.reinterpret();
    let actual_vptr = Ptr!(&pmdm_casted->vtable).load(process.memory())?;
    assert_eq!(actual_vptr, expected_vptr);

    Ok(())
}
