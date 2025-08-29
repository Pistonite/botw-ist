use std::sync::Arc;

use teleparse::{Span, ToSpan};

use crate::cir;
use crate::error::{ErrorReport, absorb_error};
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
    /// Multiple commands acting as one
    Multi(Vec<cir::Command>),

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
    /// See [`syn::CmdEat`]
    Eat(Vec<cir::ItemSelectSpec>),
    /// `cook` - Cook held items. See [`syn::CmdCook`]
    CookHeld,
    /// Hold items and cook them. See [`syn::CmdCook`]
    Cook(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdEntangle`]
    Entangle(Box<cir::ItemSelectSpec>),
    /// See [`syn::CmdCoTargeting`]
    CoTargeting(Box<cir::ItemSelectSpec>),
    /// See [`syn::CmdSort`]
    Sort(cir::CategorySpec),

    /// Start or stop menu overload
    Overload(bool),
    /// See [`syn::CmdSpawn`]
    Spawn(Vec<cir::ItemSpec>),

    /// Use DPad Quick Menu
    CoDpad,
    /// See [`syn::CmdEquip`]
    Equip(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdUnequip`]
    Unequip(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdUse`] and [`crate::syn::CmdShoot`], second arg is times
    Use(Box<cir::ItemNameSpec>, usize),
    /// See [`syn::CmdCoPerUse`]
    CoPerUse(i32),
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
    /// See [`syn::CmdSuRemove`]
    SuRemove(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdSuSetGdt`]
    ///
    /// First arg is flag name
    SuSetGdt(String, Box<cir::GdtMeta>),
    /// Activate arrowless smuggle (hold attach)
    SuArrowlessSmuggle,
    /// System Commands
    SuSystem(Vec<cir::SysCommand>),
    /// Init Pouch for Quest
    SuTrialStart,
    /// Restore Pouch for Quest
    SuTrialEnd,

    /// See [`syn::CmdRoast`] and [`crate::syn::CmdBake`]
    Roast(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdBoil`]
    Boil(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdFreeze`]
    Freeze(Vec<cir::ItemSelectSpec>),
}
// make sure the command size does not update unexpectedly
#[cfg(target_pointer_width = "64")]
static_assertions::assert_eq_size!(Command, [usize; 4]);
#[cfg(target_pointer_width = "32")]
static_assertions::assert_eq_size!(Command, [usize; 6]);

impl Command {
    /// Convience wrapper to create a command for setting a S32 gamedata flag
    #[inline]
    pub fn set_gdt_s32(flag_name: &str, value: i32) -> Self {
        Self::SuSetGdt(
            flag_name.to_string(),
            Box::new(cir::GdtMeta::new(cir::GdtValueSpec::S32(value), None)),
        )
    }
    #[inline]
    pub fn set_gdt_bool_array(flag_name: &str, value: bool, idx: usize) -> Self {
        Self::SuSetGdt(
            flag_name.to_string(),
            Box::new(cir::GdtMeta::new(cir::GdtValueSpec::Bool(value), Some(idx))),
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
        C::Spawn(cmd) => Some(X::Spawn(
            cir::parse_item_list_finite(&cmd.items, resolver, errors).await,
        )),
        A![PauseDuring(_)] => Some(X::CoPauseDuring),
        //////////////////////////////////////////////////////////////////
        C::OpenInv(_) => Some(X::OpenInv),
        C::CloseInv(_) => Some(X::CloseInv),
        A![Smug(_)] => Some(X::CoSmug),
        C::Hold(cmd) => Some(X::Hold(match cmd.items.as_ref() {
            Some(items) => cir::parse_item_list_constrained(items, resolver, errors).await,
            None => vec![],
        })),
        C::Unhold(_) => Some(X::Unhold),
        A![Overworld(_)] => Some(X::CoOverworld),
        C::Drop(cmd) => Some(X::Drop(match cmd.items.as_ref() {
            Some(items) => cir::parse_item_list_constrained(items, resolver, errors).await,
            None => vec![],
        })),
        C::Dnp(cmd) => Some(X::Dnp(
            cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
        )),
        C::Eat(cmd) => Some(X::Eat(
            cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
        )),
        C::Cook(cmd) => match cmd.items.as_ref() {
            None => Some(X::CookHeld),
            Some(items) => Some(X::Cook(
                cir::parse_item_list_constrained(items, resolver, errors).await,
            )),
        },
        C::Entangle(cmd) => Some(X::Entangle(Box::new(
            cir::parse_one_item_constrained(&cmd.item, resolver, errors).await?,
        ))),
        A![Targeting(cmd)] => Some(X::CoTargeting(Box::new(
            cir::parse_one_item_constrained(&cmd.item, resolver, errors).await?,
        ))),
        C::Sort(cmd) => absorb_error(
            errors,
            cir::parse_category_with_times(&cmd.category, cmd.times.as_ref()),
        )
        .map(X::Sort),
        //////////////////////////////////////////////////////////////////
        C::Overload(_) => Some(X::Overload(true)),
        C::Unoverload(_) => Some(X::Overload(false)),
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
        A![PerUse(cmd)] => {
            let amount = absorb_error(
                errors,
                cir::parse_syn_int_str_i32(&cmd.amount, cmd.amount.span()),
            )?;
            Some(X::CoPerUse(amount))
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
        C::Save(_) => Some(X::Save(None)),
        C::SaveAs(cmd) => Some(X::Save(Some(parse_save_name(&cmd.name)))),
        C::Reload(cmd) => Some(X::Reload(cmd.name.as_ref().map(parse_save_name))),
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
        C::SuRemove(cmd) => Some(X::SuRemove(
            cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
        )),
        C::SuSetGdt(cmd) => {
            let gdt_value = cir::parse_gdt_meta(&cmd.props, errors)?;
            let flag_name = cmd.flag_name.name.to_string();
            Some(X::SuSetGdt(flag_name, Box::new(gdt_value)))
        }
        C::SuArrowlessSmuggle(_) => Some(X::SuArrowlessSmuggle),
        C::SuSystem(cmd) => Some(X::SuSystem(cir::parse_system_meta(&cmd.props, errors))),
        C::SuTrialStart(_) => Some(X::SuTrialStart),
        C::SuTrialEnd(_) => Some(X::SuTrialEnd),
        //////////////////////////////////////////////////////////////////
        A![Slots(cmd)] => {
            let meta = cir::parse_slots_meta(&cmd.meta, errors);
            let mut cmds = Vec::with_capacity(3);
            if let Some(x) = meta.weapon {
                cmds.push(X::set_gdt_s32("WeaponPorchStockNum", x));
            }
            if let Some(x) = meta.bow {
                cmds.push(X::set_gdt_s32("BowPorchStockNum", x));
            }
            if let Some(x) = meta.shield {
                cmds.push(X::set_gdt_s32("ShieldPorchStockNum", x));
            }
            Some(X::Multi(cmds))
        }
        A![Discovered(cmd)] => {
            let meta = cir::parse_discover_meta(&cmd.meta, errors);
            let mut cmds = Vec::with_capacity(7);
            for (i, x) in meta.categories.into_iter().enumerate() {
                if let Some(x) = x {
                    cmds.push(X::set_gdt_bool_array("IsOpenItemCategory", x, i));
                }
            }
            Some(X::Multi(cmds))
        }

        //////////////////////////////////////////////////////////////////
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
    }
}

/// Parse a save name. Note that "empty string" is allowed
/// and is different from the manual save
fn parse_save_name(name: &syn::ItemName) -> String {
    match name {
        syn::ItemName::Word(word) => word.to_string(),
        syn::ItemName::Quoted(quoted) => quoted.as_str().trim_matches('"').to_string(),
        syn::ItemName::Angle(word) => word.name.to_string(),
    }
}

impl Command {
    /// Convert the command to script text
    pub fn to_script(&self, out: &mut String) {
        use std::fmt::Write;
        fn item_specs_to_script(command: &str, items: &[cir::ItemSpec], out: &mut String) {
            out.push_str(command);
            for item in items {
                out.push(' ');
                item.to_script(out);
            }
        }
        fn item_select_specs_to_script(
            command: &str,
            items: &[cir::ItemSelectSpec],
            out: &mut String,
        ) {
            out.push_str(command);
            for item in items {
                out.push(' ');
                item.to_script(out);
            }
        }
        match self {
            Command::Multi(commands) => {
                for c in commands {
                    c.to_script(out);
                    out.push(';');
                }
            }
            Command::CoAccuratelySimulate => out.push_str(":accurately-simulate"),
            Command::Get(items) => item_specs_to_script("get", items, out),
            Command::PickUp(items) => item_select_specs_to_script("pick-up", items, out),
            Command::CoPauseDuring => out.push_str(":pause-during"),

            Command::OpenInv => out.push_str("pause"),
            Command::CloseInv => out.push_str("unpause"),
            Command::CoSmug => out.push_str(":smug"),
            Command::Hold(items) => item_select_specs_to_script("hold", items, out),
            Command::Unhold => out.push_str("unhold"),
            Command::CoOverworld => out.push_str(":overworld"),
            Command::Drop(items) => item_select_specs_to_script("drop", items, out),
            Command::Dnp(items) => item_select_specs_to_script("dnp", items, out),
            Command::Eat(items) => item_select_specs_to_script("eat", items, out),
            Command::CookHeld => out.push_str("cook"),
            Command::Cook(items) => item_select_specs_to_script("cook", items, out),
            Command::Entangle(item) => {
                out.push_str("entangle ");
                item.to_script(out);
            }
            Command::CoTargeting(item) => {
                out.push_str(":targeting ");
                item.to_script(out);
            }
            Command::Sort(cat) => {
                write!(out, "sort {}", cat.category).unwrap();
                if cat.amount != 1 {
                    write!(out, " {} times", cat.amount).unwrap();
                }
            }

            Command::Overload(activate) => {
                if *activate {
                    out.push_str("overload");
                } else {
                    out.push_str("unoverload");
                }
            }
            Command::Spawn(items) => item_specs_to_script("spawn", items, out),

            Command::CoDpad => out.push_str(":dpad"),
            Command::Equip(items) => item_select_specs_to_script("equip", items, out),
            Command::Unequip(items) => item_select_specs_to_script("unequip", items, out),
            Command::Use(item, times) => {
                out.push_str("use ");
                item.to_script(out);
                if *times != 1 {
                    write!(out, " {times} times").unwrap();
                }
            }
            Command::CoPerUse(x) => write!(out, ":per-use {x}").unwrap(),
            Command::CoNonBreaking => out.push_str(":non-breaking"),
            Command::CoBreaking => out.push_str(":breaking"),
            Command::ThrowWeapon => out.push_str("throw weapon"),
            Command::Display(items) => item_select_specs_to_script("display", items, out),

            Command::OpenShop => out.push_str("talk-to npc"),
            Command::CloseShop => out.push_str("untalk"),
            Command::Buy(items) => item_specs_to_script("buy", items, out),
            Command::Sell(items) => item_select_specs_to_script("sell", items, out),
            Command::CoSameDialog => out.push_str(":same-dialog"),
            Command::Save(file) => {
                out.push_str("save");
                if let Some(file) = file {
                    write!(out, "-as \"{file}\"").unwrap();
                }
            }
            Command::Reload(file) => {
                out.push_str("reload");
                if let Some(file) = file {
                    write!(out, " \"{file}\"").unwrap();
                }
            }
            Command::CloseGame => out.push_str("close-game"),
            Command::NewGame => out.push_str("new-game"),
            Command::SuBreak(slots) => write!(out, "!break {slots} slots").unwrap(),
            Command::SuInit(items) => item_specs_to_script("!init", items, out),
            Command::SuAddSlot(items) => item_specs_to_script("!add-slot", items, out),
            Command::SuSwap(a, b) => {
                out.push_str("!swap ");
                a.to_script(out);
                out.push(' ');
                b.to_script(out);
            }
            Command::SuWrite(write_meta, item) => {
                out.push_str("!write ");
                write_meta.to_script(out);
                out.push_str(" to ");
                item.to_script(out);
            }
            Command::SuRemove(items) => item_select_specs_to_script("!remove", items, out),
            Command::SuSetGdt(name, meta) => {
                write!(out, "!set-gdt <{name}>").unwrap();
                meta.to_script(out);
            }
            Command::SuArrowlessSmuggle => out.push_str("!arrowless-smuggle"),
            Command::SuSystem(cmds) => {
                out.push_str("!system [");
                let mut iter = cmds.iter();
                if let Some(x) = iter.next() {
                    x.data.to_script(out);
                    for x in iter {
                        out.push(',');
                        x.data.to_script(out);
                    }
                }
                out.push(']');
            }
            Command::SuTrialStart => out.push_str("!trial-start"),
            Command::SuTrialEnd => out.push_str("!trial-end"),
            Command::Roast(items) => item_select_specs_to_script("roast", items, out),
            Command::Boil(items) => item_select_specs_to_script("boil", items, out),
            Command::Freeze(items) => item_select_specs_to_script("freeze", items, out),
        }
    }
}
