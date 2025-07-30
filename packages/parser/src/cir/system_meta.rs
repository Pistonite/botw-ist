use teleparse::{Span, tp};

use crate::cir;
use crate::error::{ErrorReport, cir_error};
use crate::syn;

use super::MetaParser;

/// System meta commands
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SysCommand {
    /// Set DLC version number (0, 1, 2, 3)
    Dlc(u8),
    /// Delete named or manual save data
    DeleteSave(Option<String>),
    /// Clear items on the ground
    ClearGround,
    /// Clear all items in the overworld, including equipped items
    ClearOverworld,
    /// Sync (re-create) player equipments in the overworld
    SyncOverworld,
    /// Load named or manual save data into GDT, but do not reload the inventory
    ReloadGdt(Option<String>),
    /// (Try) remove translucent items and trigger a loading screen
    LoadingScreen,
    /// Trigger a loading screen without first removing translucent items
    LoadingScreenNoRemoveTranslucent,
}

struct SysCommandMeta {
    commands: Vec<SysCommand>
}
impl MetaParser for SysCommandMeta {
    type Output = Vec<SysCommand>;

    fn visit_entry(
        &mut self,
        key: &tp::String<syn::MetaKey>,
        value: Option<&syn::MetaValue>,
        v_span: Span,
        errors: &mut Vec<ErrorReport>,
    ) {
        super::cir_match_meta_key_value! { (key, key_str, value, v_span, errors):
            "dlc" => required {
                int(x) => self.commands.push(SysCommand::Dlc(x.clamp(0, 3) as u8)),
                float(x) => self.commands.push(SysCommand::Dlc((x as i64).clamp(0, 3) as u8)),
                string(x) => {
                    let Some(x) = cir::enum_name::parse_dlc_version(&x) else {
                        errors.push(cir_error!())

                    };
                }
            },
        }
    }

    fn visit_end(self, _meta: &syn::Meta, _errors: &mut Vec<ErrorReport>) -> Self::Output {
        self.commands
    }
}
