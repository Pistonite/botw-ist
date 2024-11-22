#include <megaton/prelude.h>
#include <exl_hook/prelude.h>

#include <Game/UI/uiPauseMenuDataMgr.h>

#include <toolkit/tcp.hpp>

// clang-format off
hook_trampoline_(test_tcp) {
    static void Callback(void* _this, const sead::SafeString& name, uking::ui::PouchItemType type, void* lists, int value,
                    bool equipped, void* modifier,
                    bool is_inventory_load) {

        botw::tcp::sendf("Test TCP: %s\n", name.cstr());
        Orig(_this, name, type, lists, value, equipped, modifier, is_inventory_load);
    }
};
// clang-format on

extern "C" void megaton_main() {
    botw::tcp::init();
    botw::tcp::start_server(5001);
    test_tcp::InstallAtOffset(0x0096f268);
    // your code here
}
