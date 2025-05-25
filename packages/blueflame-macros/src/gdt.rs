/// Make a `gdt::FlagDescriptor` from shorthand
#[macro_export]
macro_rules! fd {
    (bool) => { blueflame::game::gdt::FdBool };
    (s32) => { blueflame::game::gdt::FdS32 };
    (f32) => { blueflame::game::gdt::FdF32 };
    (str32) => { blueflame::game::gdt::FdString32 };
    (str64) => { blueflame::game::gdt::FdString64 };
    (str256) => { blueflame::game::gdt::FdString256 };
    (vec2f) => { blueflame::game::gdt::FdVector2f };
    (vec3f) => { blueflame::game::gdt::FdVector3f };
    (vec4f) => { blueflame::game::gdt::FdVector4f };
    (bool[]) => { blueflame::game::gdt::FdBoolArray };
    (s32[]) => { blueflame::game::gdt::FdS32Array };
    (f32[]) => { blueflame::game::gdt::FdF32Array };
    (str64[]) => { blueflame::game::gdt::FdString64Array };
    (str256[]) => { blueflame::game::gdt::FdString256Array };
    (vec2f[]) => { blueflame::game::gdt::FdVector2fArray };
    (vec3f[]) => { blueflame::game::gdt::FdVector3fArray };
}
