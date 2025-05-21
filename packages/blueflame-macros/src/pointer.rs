
/// Pointer helper macro
///
/// This has 2 uses:
/// - As a type, to declare a pointer type to some struct T in emulated memory.
/// - As a value, to evaluate C-like syntax to navigate layout of T
///
/// # Examples
/// ```rust,ignore
/// Ptr![MyStruct]      // PtrToSized<MyStruct, MyStruct::SIZE>
/// Ptr![MyStruct[10]]  // PtrToArray<MyStruct, MyStruct::SIZE, 10>
/// Ptr!(& p->field)
/// Ptr!(& p->field1 .field2)
/// Ptr!(& p[i])
/// Ptr!(& p[i].field1.field2)
/// Ptr!(<T>...)   // PtrToSized::<T, T::SIZE>::new_const(...)
/// Ptr!(<T[10]>...)   // PtrToArray::<T, T::SIZE, 10>::new_const(...)
/// Ptr!(...)   // PtrToSized::new_const(...)
/// Ptr!([]...)   // PtrToArray::new_const(...)
/// ```
#[macro_export] 
macro_rules! Ptr {

    (& $ptr:ident -> $field:ident) => {{
        blueflame::memory::PtrToSized::__pointee_layout($ptr).$field.add($ptr.to_raw())
    }};

    (& $ptr:ident [ $index:literal ]) => {{
        blueflame::memory::PtrToArray::ith_const($ptr, $index)
    }};

    (& $ptr:ident [ $index:expr ]) => {{
        blueflame::memory::PtrToArray::ith($ptr, $index)
    }};

    (& $ptr:ident [ $index:expr ] $( . $rest:ident)* ) => {{
        let p = blueflame::memory::PtrToArray::ith($ptr, $index);
        blueflame::memory::Ptr!(& p -> $($rest).*)
    }};

    (& $ptr:ident -> $a:ident . $b:ident ) => {{
        let a = blueflame::memory::PtrToSized::__pointee_layout($ptr).$a.add($ptr.to_raw());
        blueflame::memory::PtrToSized::__pointee_layout(a).$b.add(a.to_raw())
    }};

    (& $ptr:ident -> $a:ident . $b:ident . $c:ident) => {{
        let a = blueflame::memory::PtrToSized::__pointee_layout($ptr).$a.add($ptr.to_raw());
        let b = blueflame::memory::PtrToSized::__pointee_layout(a).$b.add(a.to_raw());
        blueflame::memory::PtrToSized::__pointee_layout(b).$c.add(b.to_raw())
    }};

    (& $ptr:ident -> $a:ident . $b:ident . $c:ident $( . $rest:ident )* ) => {{
        let a = blueflame::memory::PtrToSized::__pointee_layout($ptr).$a.add($ptr.to_raw());
        let b = blueflame::memory::PtrToSized::__pointee_layout(a).$b.add(a.to_raw());
        let c = blueflame::memory::PtrToSized::__pointee_layout(b).$c.add(b.to_raw());
        blueflame::memory::Ptr!(& c -> $($rest).*)
    }};

    (< $t:ty > ( $value: expr )) => {
        blueflame::memory::PtrToSized::<$t, { <$t as blueflame::memory::MemObject>::SIZE }>::new_const($value)
    };

    (< $t:ty [$len:literal] > ( $value: expr )) => {
        blueflame::memory::PtrToArray::<$t, { <$t as blueflame::memory::MemObject>::SIZE }, $len>::new_const($value)
    };

    ($t:ty) => {
        blueflame::memory::PtrToSized::<$t, { <$t as blueflame::memory::MemObject>::SIZE }>
    };

    ($t:ty [ $len:literal ]) => {
        blueflame::memory::PtrToArray::<$t, { <$t as blueflame::memory::MemObject>::SIZE }, $len>
    };

    ($value:expr) => {
        blueflame::memory::PtrToSized::new_const($value)
    };

    ([] $value:expr) => {
        blueflame::memory::PtrToArray::new_const($value)
    };
}
