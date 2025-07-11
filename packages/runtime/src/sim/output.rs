use blueflame::processor::CrashReport;

use crate::error::{ErrorReport, RuntimeViewError};
use crate::{iv, sim};

#[derive(Clone, Default)]
pub struct RunOutput {
    /// State at each simulation step
    pub states: Vec<sim::State>,
    pub errors: Vec<ErrorReport>,
}

impl RunOutput {
    /// Get the pouch inventory view for the given step in the script
    ///
    /// If there are no steps in the script, an empty pouch list is returned. Otherwise,
    /// the state at the given step is used to generate the pouch list unless the step is out of
    /// bounds, in which case the last state is used.
    pub fn get_pouch_list(&self, step: usize) -> Result<iv::PouchList, RuntimeViewError> {
        let Some(state) = self.get_state_by_step(step) else {
            return Ok(Default::default());
        };
        let state = sim::view::view_game_state!(state);

        Ok(sim::view::extract_pouch_view(
            &state.process,
            &state.systems,
        )?)
    }

    /// Get the GDT inventory view for the given step in the script
    ///
    /// Trailing items with empty names are not included
    pub fn get_gdt_inventory(&self, step: usize) -> Result<iv::Gdt, RuntimeViewError> {
        let Some(state) = self.get_state_by_step(step) else {
            return Ok(Default::default());
        };
        let state = sim::view::view_game_state!(state);

        Ok(sim::view::extract_gdt_view(&state.process)?)
    }

    /// Get the overworld view for the given step in the script
    pub fn get_overworld_items(&self, step: usize) -> Result<iv::Overworld, RuntimeViewError> {
        let Some(state) = self.get_state_by_step(step) else {
            return Ok(Default::default());
        };
        let state = sim::view::view_game_state!(state);

        Ok(state.systems.overworld.to_iv())
    }

    /// Get the crash report for a step, if the game has crashed on that step
    pub fn get_crash_report(&self, step: usize) -> Option<&CrashReport> {
        if self.states.is_empty() {
            return None;
        }
        // safety: is_empty() is false so -1 will not underflow
        let mut step = step.min(self.states.len() - 1);

        loop {
            let state = self.states.get(step)?;
            match &state.game {
                sim::Game::Uninit
                | sim::Game::Running(_)
                | sim::Game::Closed
                | sim::Game::PreviousClosed => return None,
                sim::Game::Crashed(crash_report) => return Some(crash_report),
                sim::Game::PreviousCrash => {
                    if step == 0 {
                        // should be unreachable
                        break;
                    } else {
                        step -= 1;
                    }
                }
            }
        }

        None
    }

    /// Get the save names at the given step. Does not include the manual save (which doesn't have
    /// a name)
    pub fn get_save_names(&self, step: usize) -> Vec<String> {
        let Some(state) = self.get_state_by_step(step) else {
            return vec![];
        };
        state.named_saves().keys().cloned().collect()
    }

    /// Get the GDT inventory view for the save in the given step in the script
    ///
    /// If name is `None`, it uses the manual save.
    /// If the named save doesn't exist, it will give an empty GDT
    pub fn get_save_inventory(
        &self,
        step: usize,
        name: Option<&str>,
    ) -> Result<iv::Gdt, RuntimeViewError> {
        match self
            .get_state_by_step(step)
            .and_then(|x| x.save_by_name(name))
        {
            None => Ok(Default::default()),
            Some(save) => Ok(sim::view::extract_gdt_from_trigger_param(&save)?),
        }
    }

    fn get_state_by_step(&self, step: usize) -> Option<&sim::State> {
        if self.states.is_empty() {
            return None;
        }
        self.states.get(step).or_else(|| self.states.last())
    }
}
