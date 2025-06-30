use blueflame::game::PouchItem;
use blueflame::memory::Ptr;


pub fn should_go_to_next_tab(
    curr_item_ptr: Ptr![PouchItem],
    tab_i: usize,
    num_tabs: usize,
    tab_heads: &[Ptr![PouchItem]; 50]
) -> bool {
    if curr_item_ptr.is_nullptr() {
        return true;
    }
    if tab_i < num_tabs - 1 {
        let next_head = tab_heads[tab_i + 1];
        return curr_item_ptr == next_head;
    }
    false
}
