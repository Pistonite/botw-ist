use std::panic::{RefUnwindSafe, UnwindSafe};
use std::sync::Arc;

use crate::env::{Environment, GameVer};
use crate::game::{SafeString, WeaponModifierInfo};
use crate::memory::Ptr;
use crate::processor::{self, Cpu0, Cpu2, Hook, HookProvider, Process, reg};

pub struct RequestCreateWeapon;
impl GameEvent for RequestCreateWeapon {
    type TArgs = (i32, String, i32, Option<WeaponModifierInfo>);

    fn get_hook_offset(game_ver: GameVer) -> u32 {
        match game_ver {
            GameVer::X150 => 0x006669f8,
            _ => panic!("request_create_weapon::subscribe not implemented"),
        }
    }

    fn extract_args(cpu: &mut Cpu0, proc: &mut Process) -> Result<Self::TArgs, processor::Error> {
        reg! { cpu:
            w[1] => let slot_idx: i32,
            x[2] => let name_ptr: Ptr![SafeString],
            w[3] => let value: i32,
            x[4] => let modifier_ptr: Ptr![WeaponModifierInfo],
        };
        let m = proc.memory();
        let name = name_ptr.cstr(m)?.load_utf8_lossy(m)?;
        let modifier = if modifier_ptr.is_nullptr() {
            None
        } else {
            Some(modifier_ptr.load(m)?)
        };
        Ok((slot_idx, name, value, modifier))
    }
}

pub trait GameEvent: Send + Sync + UnwindSafe + RefUnwindSafe + 'static {
    type TArgs;
    fn get_hook_offset(game_ver: GameVer) -> u32;
    fn extract_args(cpu: &mut Cpu0, proc: &mut Process) -> Result<Self::TArgs, processor::Error>;
    fn execute_subscribed<T, F>(
        cpu: &mut Cpu2<'_, '_>,
        state: T,
        listener: fn(&T, Self::TArgs),
        mut execute: F,
    ) -> Result<T, processor::Error>
    where
        T: Send + Sync + UnwindSafe + RefUnwindSafe + 'static,
        F: FnMut() -> Result<(), processor::Error>,
        Self: Sized,
    {
        let state = Arc::new(state);
        let hook: GameEventHook<T, Self> = GameEventHook {
            state: Arc::clone(&state),
            listener,
        };
        let hook = Arc::new(hook);
        GameEventHook::register(&hook, cpu);
        let result = execute();
        hook.unregister(cpu);
        drop(hook); // this should drop down the ref count to 1

        result.map(|_| Arc::into_inner(state).expect("ref count not 1 in execute_subscribed"))
    }
}

struct GameEventHook<T, TEvent: GameEvent> {
    state: Arc<T>,
    listener: fn(&T, TEvent::TArgs),
}
impl<T: Send + Sync + UnwindSafe + RefUnwindSafe + 'static, TEvent: GameEvent> HookProvider
    for GameEventHook<T, TEvent>
{
    fn fetch(&self, main_offset: u32, env: Environment) -> Result<Option<Hook>, processor::Error> {
        if self.is_hook_offset(env.game_ver, main_offset) {
            let state = Arc::clone(&self.state);
            let listener = self.listener;
            Ok(Some(Hook::Start(processor::box_execute(
                move |cpu, proc| {
                    let args = TEvent::extract_args(cpu, proc)?;
                    listener(&state, args);
                    Ok(())
                },
            ))))
        } else {
            Ok(None)
        }
    }
}
impl<T: Send + Sync + UnwindSafe + RefUnwindSafe + 'static, TEvent: GameEvent>
    GameEventHook<T, TEvent>
{
    pub fn register(s: &Arc<Self>, cpu: &mut Cpu2<'_, '_>) {
        let s2 = Arc::clone(s);
        cpu.push_hooks(s2, |game_ver, main_offset, _size| {
            s.is_hook_offset(game_ver, main_offset)
        });
    }
    pub fn unregister(&self, cpu: &mut Cpu2<'_, '_>) {
        cpu.pop_hooks(move |game_ver, main_offset, _size| {
            self.is_hook_offset(game_ver, main_offset)
        });
    }
    pub fn is_hook_offset(&self, game_ver: GameVer, main_offset: u32) -> bool {
        main_offset == TEvent::get_hook_offset(game_ver)
    }
}
