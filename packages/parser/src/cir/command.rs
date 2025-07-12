use std::sync::Arc;

use teleparse::{Span, ToSpan};

use crate::cir;
use crate::error::{ErrorReport, absorb_error, cir_error};
use crate::search::QuotedItemResolver;
use crate::syn;

/// A simulation step
#[derive(Debug, Clone)]
pub struct Step {
    /// The command to be executed
    pub command: CommandWithSpan,

    /// The notes associated with this step
    /// Note many steps can share the same note
    pub notes: Arc<str>,
}

impl Step {
    pub fn new(span: Span, command: cir::Command, notes: Arc<str>) -> Self {
        Self {
            command: CommandWithSpan { command, span },
            notes,
        }
    }

    /// Get the start byte position of the step in source script
    pub fn pos(&self) -> usize {
        self.command.span.lo
    }

    pub fn span(&self) -> Span {
        self.command.span
    }

    pub fn command(&self) -> &Command {
        &self.command.command
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommandWithSpan {
    command: Command,
    span: Span,
}

/// The command to be executed in the simulator
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Command {
    /// Disable performance optimization that may be inaccurate
    CoAccuratelySimulate,
    /// See [`syn::CmdGet`]
    Get(Vec<cir::ItemSpec>),
    /// See [`syn::CmdPickUp`]
    PickUp(Vec<cir::ItemSelectSpec>),
    /// `:pause-during` annotation.
    ///
    /// - Get: Assumes an item text box will appear for the next command,
    ///   and open the pause menu during such text box
    /// - Throw/Display/OvDrop: Delay removal of the weapon til after dialog closes
    ///   (for making translucent items)
    CoPauseDuring,

    /// See [`syn::CmdOpenInv`]
    OpenInv,
    /// See [`syn::CmdCloseInv`]
    CloseInv,
    /// `:smug` annotation.
    ///
    /// Perform item smuggle for arrowless offset on next hold
    CoSmug,
    /// See [`syn::CmdHold`]
    Hold(Vec<cir::ItemSelectSpec>),
    /// `unhold`
    Unhold,
    /// Specify an option to be done in overworld
    CoOverworld,
    /// See [`syn::CmdDrop`] - Items are additional items to hold before dropping
    Drop(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdDnp`]
    Dnp(Vec<cir::ItemSelectSpec>),
    /// `cook` - Cook held items. See [`syn::CmdCook`]
    CookHeld,
    /// Hold items and cook them. See [`syn::CmdCook`]
    Cook(Vec<cir::ItemSelectSpec>),

    /// Use DPad Quick Menu
    CoDpad,
    /// See [`syn::CmdEquip`]
    Equip(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdUnequip`]
    Unequip(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdUse`] and [`crate::syn::CmdShoot`]
    ///
    /// Second arg is times
    Use(Box<cir::ItemNameSpec>, usize),
    /// Specify the throwing action should not break the weapon
    CoNonBreaking,
    /// Specify the throwing action should break the weapon
    CoBreaking,
    /// `throw weapon`
    ThrowWeapon,
    /// See [`syn::CmdDisplay`]
    Display(Vec<cir::ItemSelectSpec>),

    /// See [`syn::CmdOpenShop`]
    OpenShop,
    /// See [`syn::CmdCloseShop`]
    CloseShop,
    /// See [`syn::CmdBuy`]
    Buy(Vec<cir::ItemSpec>),
    /// See [`syn::CmdSell`]
    Sell(Vec<cir::ItemSelectSpec>),
    /// `:same-dialog` annotation.
    ///
    /// Buy from the same NPC right after selling without
    /// returning to overworld
    CoSameDialog,

    /// See [`syn::CmdEntangle`]
    Entangle(Box<cir::ItemSelectSpec>),
    /// See [`syn::CmdTargeting`]
    CoTargeting(Box<cir::ItemSelectSpec>),

    /// `save` - make a manual save or named save, see [`syn::CmdSaveAs`]
    Save(Option<String>),
    /// `reload` - Load manual save or named save, see [`syn::CmdReload`]
    Reload(Option<String>),
    /// `close-game` - Close the game
    CloseGame,
    /// `new-game` - Start a new game
    NewGame,

    /// See [`syn::CmdSuBreak`]
    SuBreak(i32),
    /// See [`syn::CmdSuInit`]
    SuInit(Vec<cir::ItemSpec>),
    /// See [`syn::CmdSuAddSlot`]
    SuAddSlot(Vec<cir::ItemSpec>),
    /// See [`syn::CmdSuSwap`]
    SuSwap(Box<cir::ItemSelectSpec>, Box<cir::ItemSelectSpec>),
    /// See [`syn::CmdSuWrite`]
    SuWrite(Box<cir::ItemMeta>, Box<cir::ItemSelectSpec>),
    /// See [`syn::CmdSuWriteName`]
    SuWriteName(Box<String>, Box<cir::ItemSelectSpec>),
    /// See [`syn::CmdSuRemove`]
    SuRemove(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdSuReloadGdt`]
    SuReloadGdt(Option<String>),
    /// `!reset-ground`
    SuResetGround,
    /// `!reset-overworld`
    SuResetOverworld,
    /// `!loading-screen`
    SuLoadingScreen,
    /// See [`syn::CmdSuSetGdt`] and [`syn::CmdSuSetGdtStr`]
    ///
    /// First arg is flag name
    SuSetGdt(String, Box<cir::GdtMeta>),

    /// See [`syn::CmdEat`]
    Eat(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdRoast`] and [`crate::syn::CmdBake`]
    Roast(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdBoil`]
    Boil(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdFreeze`]
    Freeze(Vec<cir::ItemSelectSpec>),

    /// See [`syn::CmdUnequip`]
    Sort(cir::CategorySpec),

    /// See [`syn::CmdEnter`]
    Enter(cir::Trial),
    /// `exit` - Exit the current trial
    Exit,
    /// `leave` - Leave the current trial without clearing it
    Leave,
}
// make sure the command size does not update unexpectedly
// size only valid for 64-bit platforms
#[cfg(not(feature = "wasm"))]
static_assertions::assert_eq_size!(Command, [u8; 0x20]);

impl Command {
    /// Convience wrapper to create a command for setting a S32 gamedata flag
    #[inline]
    pub fn set_gdt_s32(flag_name: &str, value: i32) -> Self {
        Self::SuSetGdt(
            flag_name.to_string(),
            Box::new(cir::GdtMeta::new(cir::GdtValue::S32(value), None)),
        )
    }
}

macro_rules! A {
    ($ident:ident (_) ) => {
        syn::Command::Annotation(syn::AnnotationCommand { annotation: syn::Annotation::$ident(_), .. })
    };
    ($ident:ident ($($arg:tt)* ) ) => {
        syn::Command::Annotation(syn::AnnotationCommand { annotation: syn::Annotation::$ident($($arg)*), .. })
    };
}

pub async fn parse_command<R: QuotedItemResolver>(
    command: &syn::Command,
    resolver: &R,
    errors: &mut Vec<ErrorReport>,
) -> Option<cir::Command> {
    use cir::Command as X;
    use syn::Command as C;
    match command {
        A![AccuratelySimulate(_)] => Some(X::CoAccuratelySimulate),
        C::Get(cmd) => Some(X::Get(
            cir::parse_item_list_finite(&cmd.items, resolver, errors).await,
        )),
        C::PickUp(cmd) => Some(X::PickUp(
            cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
        )),
        A![PauseDuring(_)] => Some(X::CoPauseDuring),
        //////////////////////////////////////////////////////////////////
        C::OpenInv(_) => Some(X::OpenInv),
        C::CloseInv(_) => Some(X::CloseInv),
        A![Smug(_)] => Some(X::CoSmug),
        C::Hold(cmd) => Some(X::Hold(
            cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
        )),
        C::Unhold(_) => Some(X::Unhold),
        A![Overworld(_)] => Some(X::CoOverworld),
        C::Drop(cmd) => Some(X::Drop(match cmd.items.as_ref() {
            Some(items) => cir::parse_item_list_constrained(items, resolver, errors).await,
            None => vec![],
        })),
        C::Dnp(cmd) => Some(X::Dnp(
            cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
        )),
        C::Cook(cmd) => match cmd.items.as_ref() {
            None => Some(X::CookHeld),
            Some(items) => Some(X::Cook(
                cir::parse_item_list_constrained(items, resolver, errors).await,
            )),
        },
        //////////////////////////////////////////////////////////////////
        A![Dpad(_)] => Some(X::CoDpad),
        C::Equip(cmd) => Some(X::Equip(
            cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
        )),
        C::Unequip(cmd) => Some(X::Unequip(
            cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
        )),
        C::Use(cmd) => {
            let times = absorb_error(errors, cir::parse_times_clause(cmd.times.as_ref()))?;
            let item = cir::parse_item_or_category_name(&cmd.item, resolver, errors).await?;
            Some(X::Use(Box::new(item), times as usize))
        }
        C::Shoot(cmd) => {
            let times = absorb_error(errors, cir::parse_times_clause(cmd.times.as_ref()))?;
            Some(X::Use(
                Box::new(cir::ItemNameSpec::Category(cir::Category::Bow)),
                times as usize,
            ))
        }
        A![NonBreaking(_)] => Some(X::CoNonBreaking),
        A![Breaking(_)] => Some(X::CoBreaking),
        C::ThrowWeapon(_) => Some(X::ThrowWeapon),
        C::Display(cmd) => Some(X::Display(
            cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
        )),
        //////////////////////////////////////////////////////////////////
        C::OpenShop(_) => Some(X::OpenShop),
        C::CloseShop(_) => Some(X::CloseShop),
        C::Buy(cmd) => Some(X::Buy(
            cir::parse_item_list_finite(&cmd.items, resolver, errors).await,
        )),
        C::Sell(cmd) => Some(X::Sell(
            cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
        )),
        A![SameDialog(_)] => Some(X::CoSameDialog),
        //////////////////////////////////////////////////////////////////
        C::Entangle(cmd) => Some(X::Entangle(Box::new(
            cir::parse_one_item_constrained(&cmd.item, resolver, errors).await?,
        ))),
        A![Targeting(cmd)] => Some(X::CoTargeting(Box::new(
            cir::parse_one_item_constrained(&cmd.item, resolver, errors).await?,
        ))),
        //////////////////////////////////////////////////////////////////
        C::Save(_) => Some(X::Save(None)),
        C::SaveAs(cmd) => Some(X::Save(Some(cmd.name.to_string()))),
        C::Reload(cmd) => Some(X::Reload(cmd.name.as_ref().map(|x| x.to_string()))),
        C::CloseGame(_) => Some(X::CloseGame),
        C::NewGame(_) => Some(X::NewGame),
        //////////////////////////////////////////////////////////////////
        C::SuBreak(cmd) => absorb_error(
            errors,
            cir::parse_syn_int_str_i32(&cmd.amount, cmd.amount.span()),
        )
        .map(X::SuBreak),
        C::SuInit(cmd) => Some(X::SuInit(
            cir::parse_item_list_finite_optional(&cmd.items, resolver, errors).await,
        )),
        C::SuAddSlot(cmd) => Some(X::SuAddSlot(
            cir::parse_item_list_finite_optional(&cmd.items, resolver, errors).await,
        )),
        C::SuSwap(cmd) => {
            let item1 = cir::parse_one_item_constrained(&cmd.item1, resolver, errors).await?;
            let item2 = cir::parse_one_item_constrained(&cmd.item2, resolver, errors).await?;
            Some(X::SuSwap(Box::new(item1), Box::new(item2)))
        }
        C::SuWrite(cmd) => {
            let meta = cir::ItemMeta::parse_syn(&cmd.props, errors);
            let item = cir::parse_one_item_constrained(&cmd.item, resolver, errors).await?;
            Some(X::SuWrite(Box::new(meta), Box::new(item)))
        }
        C::SuWriteName(cmd) => {
            let name = cmd.name.name.to_string();
            let item = cir::parse_one_item_constrained(&cmd.item, resolver, errors).await?;
            Some(X::SuWriteName(Box::new(name), Box::new(item)))
        }
        C::SuRemove(cmd) => Some(X::SuRemove(
            cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
        )),
        C::SuReloadGdt(cmd) => Some(X::SuReloadGdt(cmd.name.as_ref().map(|x| x.to_string()))),
        C::SuResetGround(_) => Some(X::SuResetGround),
        C::SuResetOverworld(_) => Some(X::SuResetOverworld),
        C::SuLoadingScreen(_) => Some(X::SuLoadingScreen),
        C::SuSetGdt(cmd) => {
            let gdt_value = cir::parse_gdt_meta(&cmd.props, errors)?;
            let flag_name = cmd.flag_name.name.to_string();
            Some(X::SuSetGdt(flag_name, Box::new(gdt_value)))
        }
        C::SuSetGdtStr(cmd) => {
            let gdt_value = cir::parse_gdt_meta_str(&cmd.props, errors, &cmd.value)?;
            let flag_name = cmd.flag_name.name.to_string();
            Some(X::SuSetGdt(flag_name, Box::new(gdt_value)))
        }

        //////////////////////////////////////////////////////////////////
        syn::Command::Eat(cmd) => Some(cir::Command::Eat(
            cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
        )),
        syn::Command::Roast(cmd) => Some(cir::Command::Roast(
            cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
        )),
        syn::Command::Bake(cmd) => {
            // note: alias for Roast
            Some(cir::Command::Roast(
                cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
            ))
        }
        syn::Command::Boil(cmd) => Some(cir::Command::Boil(
            cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
        )),
        syn::Command::Freeze(cmd) => Some(cir::Command::Freeze(
            cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
        )),

        syn::Command::Sort(cmd) => {
            match cir::parse_category_with_times(&cmd.category, cmd.times.as_ref()) {
                Ok(spec) => Some(cir::Command::Sort(spec)),
                Err(e) => {
                    errors.push(e);
                    None
                }
            }
        }

        syn::Command::Enter(cmd) => match cir::parse_trial(&cmd.trial, &cmd.trial.span()) {
            Ok(trial) => Some(cir::Command::Enter(trial)),
            Err(e) => {
                errors.push(e);
                None
            }
        },
        syn::Command::Exit(_) => Some(cir::Command::Exit),
        syn::Command::Leave(_) => Some(cir::Command::Leave),

        A![WeaponSlots(cmd)] => {
            let slots = absorb_error(
                errors,
                cir::parse_syn_int_str_i32(&cmd.amount, cmd.amount.span()),
            )?;
            if slots < 8 || slots > 20 {
                errors.push(cir_error!(
                    &cmd.amount,
                    InvalidEquipmentSlotNum(cir::Category::Weapon, slots)
                ));
                return None;
            }
            Some(X::set_gdt_s32("WeaponPorchStockNum", slots))
        }
        A![BowSlots(cmd)] => {
            let slots = absorb_error(
                errors,
                cir::parse_syn_int_str_i32(&cmd.amount, cmd.amount.span()),
            )?;
            if slots < 5 || slots > 14 {
                errors.push(cir_error!(
                    &cmd.amount,
                    InvalidEquipmentSlotNum(cir::Category::Bow, slots)
                ));
                return None;
            }
            Some(X::set_gdt_s32("BowPorchStockNum", slots))
        }
        A![ShieldSlots(cmd)] => {
            let slots = absorb_error(
                errors,
                cir::parse_syn_int_str_i32(&cmd.amount, cmd.amount.span()),
            )?;
            if slots < 4 || slots > 20 {
                errors.push(cir_error!(
                    &cmd.amount,
                    InvalidEquipmentSlotNum(cir::Category::Shield, slots)
                ));
                return None;
            }
            Some(X::set_gdt_s32("ShieldPorchStockNum", slots))
        }
    }
}
