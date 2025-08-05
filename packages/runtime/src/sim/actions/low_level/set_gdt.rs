use std::sync::Arc;

use blueflame::game::gdt;
use blueflame::memory::{self, proxy};
use blueflame::processor::Cpu2;
use skybook_parser::cir;

use crate::error::{ErrorReport, sim_error, sim_warning};
use crate::sim;

/// Set a gamedata flag
pub fn set_gdt(
    ctx: &mut sim::Context<&mut Cpu2>,
    name: &str,
    meta: &cir::GdtMeta,
    errors: &mut Vec<ErrorReport>,
) -> Result<(), memory::Error> {
    let span = ctx.span;

    macro_rules! cannot_find {
        ($desc:literal) => {{
            errors.push(sim_error!(
                span,
                CannotFindGdtFlag(name.to_string(), $desc.to_string())
            ));
            return Ok(());
        }};
    }
    macro_rules! invalid_index {
        ($i:ident, $desc:literal) => {{
            errors.push(sim_warning!(
                span,
                InvalidGdtArrayIndex(name.to_string(), $desc.to_string(), $i)
            ));
            return Ok(());
        }};
    }
    let m = ctx.cpu().proc.memory();
    let gdt_ptr = gdt::trigger_param_ptr(m)?;
    let proc = &mut ctx.cpu().proc;
    proxy! { let mut gdt = *gdt_ptr as trigger_param in proc};

    match &meta.value {
        cir::GdtValueSpec::Bool(v) => match meta.array_idx {
            Some(i) => match gdt.by_name_mut::<gdt::fd!(bool[])>(name) {
                None => cannot_find!("bool[]"),
                Some(flag) => {
                    if !flag.set_at(i, *v) {
                        invalid_index!(i, "bool[]");
                    }
                }
            },
            None => match gdt.by_name_mut::<gdt::fd!(bool)>(name) {
                None => cannot_find!("bool[]"),
                Some(flag) => flag.set(*v),
            },
        },
        cir::GdtValueSpec::S32(v) => match meta.array_idx {
            Some(i) => match gdt.by_name_mut::<gdt::fd!(s32[])>(name) {
                None => cannot_find!("s32[]"),
                Some(flag) => {
                    if !flag.set_at(i, *v) {
                        invalid_index!(i, "s32[]");
                    }
                }
            },
            None => match gdt.by_name_mut::<gdt::fd!(s32)>(name) {
                None => cannot_find!("s32"),
                Some(flag) => flag.set(*v),
            },
        },
        cir::GdtValueSpec::F32(v) => match meta.array_idx {
            Some(i) => match gdt.by_name_mut::<gdt::fd!(f32[])>(name) {
                None => cannot_find!("f32[]"),
                Some(flag) => {
                    if !flag.set_at(i, *v) {
                        invalid_index!(i, "f32[]");
                    }
                }
            },
            None => match gdt.by_name_mut::<gdt::fd!(f32)>(name) {
                None => cannot_find!("f32"),
                Some(flag) => flag.set(*v),
            },
        },
        cir::GdtValueSpec::String32(v) => match meta.array_idx {
            // there are no str32[] flags in the game
            Some(_) => cannot_find!("str32[]"),
            None => match gdt.by_name_mut::<gdt::fd!(str32)>(name) {
                None => cannot_find!("str32"),
                Some(flag) => flag.set(Arc::from(v.as_str())),
            },
        },
        cir::GdtValueSpec::String64(v) => match meta.array_idx {
            Some(i) => match gdt.by_name_mut::<gdt::fd!(str64[])>(name) {
                None => cannot_find!("str64[]"),
                Some(flag) => {
                    if !flag.set_at(i, Arc::from(v.as_str())) {
                        invalid_index!(i, "str64[]")
                    }
                }
            },
            None => match gdt.by_name_mut::<gdt::fd!(str64)>(name) {
                None => cannot_find!("str64"),
                Some(flag) => flag.set(Arc::from(v.as_str())),
            },
        },
        cir::GdtValueSpec::String256(v) => match meta.array_idx {
            Some(i) => match gdt.by_name_mut::<gdt::fd!(str256[])>(name) {
                None => cannot_find!("str256[]"),
                Some(flag) => {
                    if !flag.set_at(i, Arc::from(v.as_str())) {
                        invalid_index!(i, "str256[]")
                    }
                }
            },
            None => match gdt.by_name_mut::<gdt::fd!(str256)>(name) {
                None => cannot_find!("str256"),
                Some(flag) => flag.set(Arc::from(v.as_str())),
            },
        },
        cir::GdtValueSpec::Vec2f(x, y) => match meta.array_idx {
            Some(i) => match gdt.by_name_mut::<gdt::fd!(vec2f[])>(name) {
                None => cannot_find!("vec2f[]"),
                Some(flag) => {
                    let (x, y) = match (x, y) {
                        (Some(x), Some(y)) => (*x, *y),
                        (x, y) => {
                            let Some(v) = flag.get_at(i) else {
                                invalid_index!(i, "vec2f[]");
                            };
                            ((*x).unwrap_or(v.0), (*y).unwrap_or(v.1))
                        }
                    };
                    if !flag.set_at(i, (x, y)) {
                        invalid_index!(i, "vec2f[]");
                    }
                }
            },
            None => match gdt.by_name_mut::<gdt::fd!(vec2f)>(name) {
                None => cannot_find!("vec2f"),
                Some(flag) => {
                    let (x, y) = match (x, y) {
                        (Some(x), Some(y)) => (*x, *y),
                        (x, y) => {
                            let v = flag.get();
                            ((*x).unwrap_or(v.0), (*y).unwrap_or(v.1))
                        }
                    };
                    flag.set((x, y));
                }
            },
        },
        cir::GdtValueSpec::Vec3f(x, y, z) => match meta.array_idx {
            Some(i) => match gdt.by_name_mut::<gdt::fd!(vec3f[])>(name) {
                None => cannot_find!("vec3f[]"),
                Some(flag) => {
                    let (x, y, z) = match (x, y, z) {
                        (Some(x), Some(y), Some(z)) => (*x, *y, *z),
                        (x, y, z) => {
                            let Some(v) = flag.get_at(i) else {
                                invalid_index!(i, "vec3f[]");
                            };
                            (
                                (*x).unwrap_or(v.0),
                                (*y).unwrap_or(v.1),
                                (*z).unwrap_or(v.2),
                            )
                        }
                    };
                    if !flag.set_at(i, (x, y, z)) {
                        invalid_index!(i, "vec3f[]");
                    }
                }
            },
            None => match gdt.by_name_mut::<gdt::fd!(vec3f)>(name) {
                None => cannot_find!("vec3f"),
                Some(flag) => {
                    let (x, y, z) = match (x, y, z) {
                        (Some(x), Some(y), Some(z)) => (*x, *y, *z),
                        (x, y, z) => {
                            let v = flag.get();
                            (
                                (*x).unwrap_or(v.0),
                                (*y).unwrap_or(v.1),
                                (*z).unwrap_or(v.2),
                            )
                        }
                    };
                    flag.set((x, y, z));
                }
            },
        },
    }

    Ok(())
}
