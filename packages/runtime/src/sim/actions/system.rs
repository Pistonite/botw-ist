use std::collections::BTreeMap;
use std::sync::Arc;

use blueflame::game::{gdt, singleton_instance};
use blueflame::processor::{self, Cpu2};
use skybook_parser::cir;

use crate::error::{ErrorReport, sim_error};
use crate::sim;

// TODO: probably better ways to handle this, when we fix the scheduling
pub fn exec_sys_commands(
    ctx: &mut sim::Context<&mut Cpu2>,
    sys: &mut sim::GameSystems,
    errors: &mut Vec<ErrorReport>,
    cmds: &[cir::SysCommand],
    saves: &mut BTreeMap<String, Arc<gdt::TriggerParam>>,
    manual_save: &mut Option<Arc<gdt::TriggerParam>>,
    dlc_version: &mut Option<u32>,
) -> Result<(), processor::Error> {
    for cmd in cmds {
        use cir::SysCommandData as S;
        match &cmd.data {
            S::Dlc(ver) => {
                let ver = *ver as u32;
                let m = ctx.cpu().proc.memory();
                let aocm = singleton_instance!(aocm(m))?;
                let m = ctx.cpu().proc.memory_mut();
                aocm.set_dlc_version(ver, m)?;
                *dlc_version = Some(ver);
            }
            S::DeleteSave(name) => match name {
                None => { *manual_save = None; }
                Some(x) => { saves.remove(x); }
            }
            S::ClearGround => sys.overworld.destroy_ground(),
            S::ClearOverworld => sys.overworld.destroy_all(),
            S::SyncOverworld => super::recreate_overworld_equipments(ctx, sys)?,
            S::ReloadGdt(name) =>  {
                let save_data = match name {
                    None => {
                        let Some(s) = manual_save.as_ref() else {
                            errors.push(sim_error!(cmd.span, NoManualSave));
                            continue;
                        };
                        s
                    },
                    Some(name) => {
                        let Some(save) = saves.get(name) else {
                            errors.push(sim_error!(cmd.span, SaveNotFound(name.to_string())));
                            continue;
                        };
                        save
                    }
                };
                super::reload_gdt(ctx, errors, save_data)?;
            },
            S::LoadingScreen => {
                super::regen_stage_internal(ctx, sys, errors, true, None)?;
            }
            S::LoadingScreenNoRemoveTranslucent => {
                super::regen_stage_internal(ctx, sys, errors, false, None)?;
            }
        }
    }
    Ok(())
}
