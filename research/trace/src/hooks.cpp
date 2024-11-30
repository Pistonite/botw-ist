#include <megaton/hook.h>

#include "hooks.hpp"
#include "reporter.hpp"
#define private public
#include <Game/UI/uiPauseMenuDataMgr.h>
#include <Game/Actor/actWeapon.h>
#undef private

using namespace uking::ui;
using namespace uking::act;
namespace sead {
class CriticalSection;
}

namespace botw::ist::trace {

//////// Pmdm Init
namespace pmdm {

static sead::CriticalSection* pmdm_mutex = nullptr;

struct hook_trampoline_(createInstance) {
    target_offset_(0x0096B1CC)
    static PauseMenuDataMgr* call(void* heap) {
        auto p = call_original(heap);
        pmdm_mutex = &p->mCritSection;
        current_reporter().send("pmdm::createInstance");
        return p;
    }
};

}

//////// sead::CriticalSection
namespace seadcs {
struct hook_trampoline_(lock) {
    target_offset_(0x00B1898C)
    static void call(sead::CriticalSection* _this) {
        if (_this == pmdm::pmdm_mutex) {
            auto& reporter = current_reporter();
            if (!reporter.is_top()) {
                reporter.send("pmdm::(mutex lock)");
            }
        }
        call_original(_this);
    }
};
struct hook_trampoline_(unlock) {
    target_offset_(0x00B1899C)
    static void call(sead::CriticalSection* _this) {
        if (_this == pmdm::pmdm_mutex) {
            auto& reporter = current_reporter();
            if (!reporter.is_top()) {
                reporter.send("pmdm::(mutex unlock)");
            }
        }
        call_original(_this);
    }
};
}

struct hook_trampoline_(doGetItem) {
    target_offset_(0x0073A464)
    static void call(const sead::SafeString& name, const WeaponModifierInfo* modifier) {
        auto& reporter = current_reporter();
        auto scope = reporter.scope("doGetItem");
        if (modifier != nullptr) {
            reporter.sendf("args:\nname=%s\nmodifier_flags=0x%x\nmodifier_value=%d", 
                           name.cstr(), modifier->flags.getDirect(), modifier->value);
        } else {
            reporter.sendf("args:\nname=%s\nmodifier=null", name.cstr());
        }
        call_original(name, modifier);
    }
};

struct hook_trampoline_(setItemDataToPouch) {
    target_offset_(0x0073AA68)
    static void call(const sead::SafeString& name, const WeaponModifierInfo* modifier) {
        auto& reporter = current_reporter();
        auto scope = reporter.scope("setItemDataToPouch");
        if (modifier != nullptr) {
            reporter.sendf("args:\nname=%s\nmodifier_flags=0x%x\nmodifier_value=%d", 
                           name.cstr(), modifier->flags.getDirect(), modifier->value);
        } else {
            reporter.sendf("args:\nname=%s\nmodifier=null", name.cstr());
        }
        call_original(name, modifier);
    }
};

//////// Inventory stuff
namespace pmdm {

struct hook_trampoline_(loadFromGameData) {
    target_offset_(0x0096BE24)
    static void call(PauseMenuDataMgr* _this) {
        auto& reporter = current_reporter();
        auto scope = reporter.scope("pmdm::loadFromGameData");
        call_original(_this);
    }
};

struct hook_trampoline_(doLoadFromGameData) {
    target_offset_(0x0096C010)
    static void call(PauseMenuDataMgr* _this) {
        current_reporter().send("pmdm::doLoadFromGameData");
        call_original(_this);
    }
};

struct hook_trampoline_(saveToGameData) {
    target_offset_(0x0096F9BC)
    static void call(PauseMenuDataMgr* _this, const sead::OffsetList<PouchItem>& list) {
        current_reporter().send("pmdm::saveToGameData");
        call_original(_this, list);
    }

};

struct hook_trampoline_(updateInventoryInfo) {
    target_offset_(0x0096C954)
    static void call(PauseMenuDataMgr* _this, void* list) {
        current_reporter().send("pmdm::updateInventoryInfo");
        call_original(_this, list);
    }
};

struct hook_trampoline_(itemGet) {
    target_offset_(0x0096EFB8)
    static void call(PauseMenuDataMgr* _this, const sead::SafeString& name, int value, const WeaponModifierInfo* modifier) {
        auto& reporter = current_reporter();
        auto scope = reporter.scope("pmdm::itemGet");
        if (modifier != nullptr) {
            reporter.sendf("args:\nname=%s\nvalue=%d\nmodifier_flags=0x%x\nmodifier_value=%d", 
                           name.cstr(), value, modifier->flags.getDirect(), modifier->value);
        } else {
            reporter.sendf("args:\nname=%s\nvalue=%d\nmodifier=null", name.cstr(), value);
        }
        call_original(_this, name, value, modifier);
    }
};

struct hook_trampoline_(autoEquipLastAddedItem) {
    target_offset_(0x00970264)
    static void call(PauseMenuDataMgr* _this) {
        current_reporter().send("pmdm::autoEquipLastAddedItem");
        call_original(_this);
    }
};

}

/////// Pouch
namespace pmdm {

struct hook_trampoline_(addToPouch) {
    target_offset_(0x0096F268)
    static void call(
        void* _this, const sead::SafeString& name, 
        PouchItemType type, void* lists, int value,
                    bool equipped, void* modifier,
                    bool is_inventory_load) {

        auto& reporter = current_reporter();
        auto scope = reporter.scope("pmdm::addToPouch");
        const char* type_str;
        switch (type) {
            case PouchItemType::Sword: type_str = "Sword"; break;
            case PouchItemType::Bow: type_str = "Bow"; break;
            case PouchItemType::Arrow: type_str = "Arrow"; break;
            case PouchItemType::Shield: type_str = "Shield"; break;
            case PouchItemType::ArmorHead: type_str = "ArmorHead"; break;
            case PouchItemType::ArmorUpper: type_str = "ArmorUpper"; break;
            case PouchItemType::ArmorLower: type_str = "ArmorLower"; break;
            case PouchItemType::Material: type_str = "Material"; break;
            case PouchItemType::Food: type_str = "Food"; break;
            case PouchItemType::KeyItem: type_str = "KeyItem"; break;
            default: type_str = "Invalid"; break;
        }
        reporter.sendf("args:\nname=%s\ntype=%s\nvalue=%d\nequipped=%d\nis_load=%d", 
                       name.cstr(), 
                       type_str, 
                       value, 
                       equipped, 
                       is_inventory_load);
        call_original(_this, name, type, lists, value, equipped, modifier, is_inventory_load);
    }
};

struct hook_trampoline_(getItemCount) {
    target_offset_(0x00970F84)
    static int call(PauseMenuDataMgr* _this, const sead::SafeString& name, bool count_equipped) {
        auto& reporter = current_reporter();
        auto scope = reporter.scope("pmdm::getItemCount");
        reporter.sendf("args:\nname=%s\ncount_equipped=%d", name.cstr(), count_equipped);
        int ret_value = call_original(_this, name, count_equipped);
        reporter.sendf("return: %d", ret_value);
        return ret_value;
    }
};




}

/////// what
namespace unk {

struct hook_trampoline_(sub_007DC3FC) {
    target_offset_(0x007DC3FC)
    static void call(i64 a1) {
        auto& reporter = current_reporter();
        auto scope = reporter.scope("unk::sub_007DC3FC");
        reporter.sendf("args:\na1=0x%llx", a1);
        call_original(a1);
    }
};

struct hook_trampoline_(sub_00984CA0) {
    target_offset_(0x00984CA0)
    static void call(i64 a1) {
        auto& reporter = current_reporter();
        auto scope = reporter.scope("unk::sub_00984CA0");
        reporter.sendf("args:\na1=0x%llx", a1);
        call_original(a1);
    }
};

struct hook_trampoline_(sub_00A915D4) {
    target_offset_(0x00A915D4)
    static i64 call(void* a1, int a2, int a3, int a4) {
        auto& reporter = current_reporter();
        auto scope = reporter.scope("unk::sub_00A915D4");
        return call_original(a1, a2, a3, a4);
    }
};

struct hook_trampoline_(sub_0073A6D0) {
    target_offset_(0x0073A6D0)
    static void call(void* a1, const sead::SafeString& name) {
        auto& reporter = current_reporter();
        auto scope = reporter.scope("unk::sub_0073A6D0 (pick up from ground)");
        reporter.sendf("args:\na1=0x%llx\nname=%s", a1, name.cstr());
        call_original(a1, name);
    }
};

}

void install_hooks() {
    pmdm::createInstance::install();
    /* seadcs::lock::install(); */
    /* seadcs::unlock::install(); */
    doGetItem::install();
    setItemDataToPouch::install();

    pmdm::loadFromGameData::install();
    pmdm::doLoadFromGameData::install();
    pmdm::saveToGameData::install();
    pmdm::updateInventoryInfo::install();
    pmdm::itemGet::install();

    pmdm::addToPouch::install();
    /* pmdm::getItemCount::install(); */

    unk::sub_007DC3FC::install();
    unk::sub_00984CA0::install();
    unk::sub_00A915D4::install();
    unk::sub_0073A6D0::install();
}

}
