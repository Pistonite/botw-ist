use teleparse::ToSpan;

use crate::cir;
use crate::error::ErrorReport;
use crate::search::QuotedItemResolver;
use crate::syn;

/// The command to be executed in the simulator
#[derive(Debug, Clone)]
pub enum Command {
    /// See [`syn::CmdGet`]
    Get(Vec<cir::ItemSpec>),
    /// See [`syn::CmdBuy`]
    Buy(Vec<cir::ItemSpec>),
    /// See [`syn::CmdPickUp`]
    PickUp(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdHold`]
    Hold(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdHoldSmuggle`]
    HoldSmuggle(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdHoldAttach`]
    HoldAttach(Vec<cir::ItemSelectSpec>),
    /// `unhold`
    Unhold,
    /// `drop` - Drop held items. See [`syn::CmdDrop`]
    DropHeld,
    /// Hold items and drop them. See [`syn::CmdDrop`]
    Drop(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdDnp`]
    Dnp(Vec<cir::ItemSelectSpec>),
    /// `cook` - Cook held items. See [`syn::CmdCook`]
    CookHeld,
    /// Hold items and cook them. See [`syn::CmdCook`]
    Cook(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdEat`]
    Eat(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdSell`]
    Sell(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdEquip`]
    Equip(Box<cir::ItemSelectSpec>),
    /// See [`syn::CmdUnequip`]
    Unequip(Box<cir::ItemSelectSpec>),
    /// See [`syn::CmdUse`] and [`crate::syn::CmdShoot`]
    Use(cir::CategorySpec),
    /// See [`syn::CmdRoast`] and [`crate::syn::CmdBake`]
    Roast(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdBoil`]
    Boil(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdFreeze`]
    Freeze(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdDestroy`]
    Destroy(Vec<cir::ItemSelectSpec>),
    /// See [`syn::CmdUnequip`]
    Sort(cir::CategorySpec),
    /// See [`syn::CmdEntangle`]
    Entangle(cir::CategorySpec),
    /// `save` - make a manual save
    Save,
    /// `save-as`. See [`syn::CmdSaveAs`]
    SaveAs(String),
    /// `reload` - Load manual save
    Reload,
    /// `reload FILE`. See [`syn::CmdReload`]
    ReloadFrom(String),
    /// `close-game` - Close the game
    CloseGame,
    /// `new-game` - Start a new game
    NewGame,

    // ==== scopes ====
    OpenInv,
    CloseInv,
    Talk,
    Untalk,

    /// See [`syn::CmdEnter`]
    Enter(cir::Trial),
    /// `exit` - Exit the current trial
    Exit,
    /// `leave` - Leave the current trial without clearing it
    Leave,
}
// make sure the command size does not update unexpectedly
static_assertions::assert_eq_size!(Command, [u8; 0x20]);

pub async fn parse_command<R: QuotedItemResolver>(
    command: &syn::Command,
    resolver: &R,
    errors: &mut Vec<ErrorReport>,
) -> Option<cir::Command> {
    match command {
        syn::Command::Get(cmd) => Some(cir::Command::Get(
            cir::parse_item_list_finite(&cmd.items, resolver, errors).await,
        )),
        syn::Command::Buy(cmd) => Some(cir::Command::Buy(
            cir::parse_item_list_finite(&cmd.items, resolver, errors).await,
        )),
        syn::Command::PickUp(cmd) => Some(cir::Command::PickUp(
            cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
        )),
        syn::Command::Hold(cmd) => Some(cir::Command::Hold(
            cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
        )),
        syn::Command::HoldSmuggle(cmd) => Some(cir::Command::HoldSmuggle(
            cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
        )),
        syn::Command::HoldAttach(cmd) => Some(cir::Command::HoldAttach(
            cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
        )),
        syn::Command::Unhold(_) => Some(cir::Command::Unhold),
        syn::Command::Drop(cmd) => match cmd.items.as_ref() {
            None => Some(cir::Command::DropHeld),
            Some(items) => Some(cir::Command::Drop(
                cir::parse_item_list_constrained(items, resolver, errors).await,
            )),
        },
        syn::Command::Dnp(cmd) => Some(cir::Command::Dnp(
            cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
        )),
        syn::Command::Cook(cmd) => match cmd.items.as_ref() {
            None => Some(cir::Command::CookHeld),
            Some(items) => Some(cir::Command::Cook(
                cir::parse_item_list_constrained(items, resolver, errors).await,
            )),
        },
        syn::Command::Eat(cmd) => Some(cir::Command::Eat(
            cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
        )),
        syn::Command::Sell(cmd) => Some(cir::Command::Sell(
            cir::parse_item_list_constrained(&cmd.items, resolver, errors).await,
        )),
        syn::Command::Equip(cmd) => Some(cir::Command::Equip(Box::new(
            cir::parse_item_or_category_with_slot(&cmd.item, resolver, errors).await?,
        ))),
        syn::Command::Unequip(cmd) => Some(cir::Command::Unequip(Box::new(
            cir::parse_item_or_category_with_slot(&cmd.item, resolver, errors).await?,
        ))),
        syn::Command::Use(cmd) => {
            match cir::parse_use_category_with_times(&cmd.category, cmd.times.as_ref()) {
                Ok(spec) => Some(cir::Command::Use(spec)),
                Err(e) => {
                    errors.push(e);
                    None
                }
            }
        }
        syn::Command::Shoot(cmd) => match cir::parse_times_clause(cmd.times.as_ref()) {
            Ok(times) => Some(cir::Command::Use(cir::CategorySpec {
                category: cir::Category::Bow,
                amount: times,
                row: 0,
                col: 0,
            })),
            Err(e) => {
                errors.push(e);
                None
            }
        },
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
        syn::Command::Destroy(cmd) => Some(cir::Command::Destroy(
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
        syn::Command::Entangle(cmd) => Some(cir::Command::Entangle(cir::parse_entangle_meta(
            &cmd.category,
            cmd.meta.as_ref(),
            errors,
        ))),
        syn::Command::Save(_) => Some(cir::Command::Save),
        syn::Command::SaveAs(cmd) => Some(cir::Command::SaveAs(cmd.name.to_string())),
        syn::Command::Reload(cmd) => match cmd.name.as_ref() {
            None => Some(cir::Command::Reload),
            Some(name) => Some(cir::Command::ReloadFrom(name.to_string())),
        },
        syn::Command::CloseGame(_) => Some(cir::Command::CloseGame),
        syn::Command::NewGame(_) => Some(cir::Command::NewGame),
        syn::Command::OpenInventory(_) => Some(cir::Command::OpenInv),
        syn::Command::CloseInventory(_) => Some(cir::Command::CloseInv),
        syn::Command::TalkTo(_) => Some(cir::Command::Talk),
        syn::Command::Untalk(_) => Some(cir::Command::Untalk),
        syn::Command::Enter(cmd) => match cir::parse_trial(&cmd.trial, &cmd.trial.span()) {
            Ok(trial) => Some(cir::Command::Enter(trial)),
            Err(e) => {
                errors.push(e);
                None
            }
        },
        syn::Command::Exit(_) => Some(cir::Command::Exit),
        syn::Command::Leave(_) => Some(cir::Command::Leave),
    }
}
