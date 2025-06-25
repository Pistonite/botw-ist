
use blueflame::game::singleton_instance;
use blueflame::linker;
use blueflame::memory::mem;
use blueflame::processor::{Cpu2, Error};

pub fn hold_material(cpu: &mut Cpu2) -> Result<(), Error> {
    // should be in tab 0, slot 0
    linker::get_item(cpu, "Item_Fruit_A", None, None)?;
    assert!(linker::can_hold_another_item(cpu)?);
    linker::trash_item(cpu, 0, 0)?;

    let pmdm_ptr = singleton_instance!(pmdm(cpu.proc.memory()))?;
    let m = cpu.proc.memory();
    let grabbed_item = pmdm_ptr.grabbed_items().ith(0);
    mem! { m:
        let grabbed_item_ptr = *(&grabbed_item->mItem);
    };
    assert!(!grabbed_item_ptr.is_nullptr());
    
    // should be the last item in the buffer
    let expected_item_ptr = pmdm_ptr.item_buffer().ith(419);
    assert_eq!(grabbed_item_ptr, expected_item_ptr);
    
    mem! { m:
        let grabbed_item_value = *(&grabbed_item_ptr->mValue);
    };
    // 1 originally, after grab, becomes 0
    assert_eq!(grabbed_item_value, 0);

    Ok(())
}
