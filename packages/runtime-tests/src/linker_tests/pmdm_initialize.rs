use blueflame::game::{PauseMenuDataMgr, singleton_instance};
use blueflame::memory::Ptr;
use blueflame::processor::{Cpu2, Error};

pub fn run(cpu: &mut Cpu2) -> Result<(), Error> {
    let pmdm_actual_addr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    assert_eq!(pmdm_actual_addr.to_raw(), 0x2222200000);

    let expected_vptr = cpu.proc.main_start() + 0x02476c38;
    let pmdm_casted: Ptr![PauseMenuDataMgr] = pmdm_actual_addr.reinterpret();
    let actual_vptr = Ptr!(&pmdm_casted->vtable).load(cpu.proc.memory())?;
    assert_eq!(actual_vptr, expected_vptr);

    Ok(())
}
