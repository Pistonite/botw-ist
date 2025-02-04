use crate::cir;
use crate::error::ErrorReport;
use crate::search::QuotedItemResolver;
use crate::syn;

/// The command to be executed in the simulator
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
    Equip(cir::ItemSelectSpec),
    /// See [`syn::CmdUnequip`]
    Unequip(cir::ItemSelectSpec),
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
    TalkTo,
    Untalk,

    /// See [`syn::CmdEnter`]
    Enter(cir::Trial),
    /// `exit` - Exit the current trial
    Exit,
    /// `leave` - Leave the current trial without clearing it
    Leave,
}

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
        syn::Command::Equip(cmd) => Some(cir::Command::Equip(
            cir::parse_item_or_category_with_slot(&cmd.item, resolver, errors).await?,
        )),
        syn::Command::Unequip(cmd) => Some(cir::Command::Unequip(
            cir::parse_item_or_category_with_slot(&cmd.item, resolver, errors).await?,
        )),
        syn::Command::Use(cmd) => {
        }
        syn::Command::Shoot(cmd_shoot) => todo!(),
        syn::Command::Roast(cmd_roast) => todo!(),
        syn::Command::Bake(cmd_bake) => todo!(),
        syn::Command::Boil(cmd_boil) => todo!(),
        syn::Command::Freeze(cmd_freeze) => todo!(),
        syn::Command::Destroy(cmd_destroy) => todo!(),
        syn::Command::Sort(cmd_sort) => todo!(),
        syn::Command::Entangle(cmd_entangle) => todo!(),
        syn::Command::Save(kw_save) => todo!(),
        syn::Command::SaveAs(cmd_save_as) => todo!(),
        syn::Command::Reload(cmd_reload) => todo!(),
        syn::Command::CloseGame(kw_close_game) => todo!(),
        syn::Command::NewGame(kw_new_game) => todo!(),
        syn::Command::OpenInventory(kw_open_inventory) => todo!(),
        syn::Command::CloseInventory(kw_close_inventory) => todo!(),
        syn::Command::TalkTo(kw_talk_to) => todo!(),
        syn::Command::Untalk(kw_untalk) => todo!(),
        syn::Command::Enter(cmd_enter) => todo!(),
        syn::Command::Exit(kw_exit) => todo!(),
        syn::Command::Leave(kw_leave) => todo!(),
    }
}
