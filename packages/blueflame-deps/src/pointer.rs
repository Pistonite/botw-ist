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
        $crate::Ptr!(& p -> $($rest).*)
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
        $crate::Ptr!(& c -> $($rest).*)
    }};

    (< $t:ty > ( $value: expr )) => {
        blueflame::memory::PtrToSized::<$t, { <$t as blueflame::memory::MemObject>::SIZE }>::new_const($value)
    };

    (< $t:ty [$len:literal] > ( $value: expr )) => {
        blueflame::memory::PtrToArray::<$t, { <$t as blueflame::memory::MemObject>::SIZE }, $len>::new_const($value)
    };

    (nullptr) => {
        blueflame::memory::PtrToSized::nullptr()
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

/// Memory operation helper macro
///
/// # Example
/// ```rust,ignore
/// mem! { memory:
///     *ptr_ident = owned_value;  // store T
///     *(ptr_expr) = *borrowed_value; // store &T (dereference then store)
///     ident = *ptr_ident;       // load
///     let ident = *(ptr_expr);    // load and bind
/// };
/// ```
///
/// Use `*` to store from a borrowed reference
#[macro_export]
macro_rules! mem {
    // load
    ($mem:ident : $(;)? $local:ident = * $ptr:ident $(;)? ) => {
         $local = $ptr.load($mem)?;
    };
    ($mem:ident : $(;)? $local:ident = * ($ptr:expr) $(;)? ) => {
         $local = $crate::Ptr!($ptr).load($mem)?;
    };
    ($mem:ident : $(;)? let $local:ident = * $ptr:ident $(;)? ) => {
         let $local = $ptr.load($mem)?;
    };
    ($mem:ident : $(;)? let $local:ident = * ($ptr:expr) $(;)? ) => {
         let $local = $crate::Ptr!($ptr).load($mem)?;
    };
    ($mem:ident : $(;)? let mut $local:ident = * $ptr:ident $(;)? ) => {
         let mut $local = $ptr.load($mem)?;
    };
    ($mem:ident : $(;)? let mut $local:ident = * ($ptr:expr) $(;)? ) => {
         let mut $local = $crate::Ptr!($ptr).load($mem)?;
    };
    ($mem:ident : $(;)? $local:ident = * $ptr:ident $( ; $($rest:tt)* )? ) => {
         $local = $ptr.load($mem)?;
         $( $crate::mem!($mem : $($rest)*); )?
    };
    ($mem:ident : $(;)? $local:ident = * ($ptr:expr) $( ; $($rest:tt)* )? ) => {
         $local = $crate::Ptr!($ptr).load($mem)?;
         $( $crate::mem!($mem : $($rest)*); )?
    };
    ($mem:ident : $(;)? let $local:ident = * $ptr:ident $( ; $($rest:tt)* )? ) => {
         let $local = $ptr.load($mem)?;
         $( $crate::mem!($mem : $($rest)*); )?
    };
    ($mem:ident : $(;)? let $local:ident = * ($ptr:expr) $( ; $($rest:tt)* )? ) => {
         let $local = $crate::Ptr!($ptr).load($mem)?;
         $( $crate::mem!($mem : $($rest)*); )?
    };
    ($mem:ident : $(;)? let mut $local:ident = * $ptr:ident $( ; $($rest:tt)* )? ) => {
         let mut $local = $ptr.load($mem)?;
         $( $crate::mem!($mem : $($rest)*); )?
    };
    ($mem:ident : $(;)? let mut $local:ident = * ($ptr:expr) $( ; $($rest:tt)* )? ) => {
         let mut $local = $crate::Ptr!($ptr).load($mem)?;
         $( $crate::mem!($mem : $($rest)*); )?
    };

    // store
    ($mem:ident : $(;)? * $ptr:ident = * $value:expr $(;)? ) => {
        $ptr.store($value, $mem)?;
    };
    ($mem:ident : $(;)? * $ptr:ident = $value:expr $(;)? ) => {
         $ptr.store(&($value), $mem)?;
    };
    ($mem:ident : $(;)? * ($($ptr:tt)*) = * $value:expr $(;)? ) => {
        $crate::Ptr!($($ptr)*).store($value, $mem)?;
    };
    ($mem:ident : $(;)? * ($($ptr:tt)*) = $value:expr $(;)? ) => {
         $crate::Ptr!($($ptr)*).store(&($value), $mem)?;
    };
    ($mem:ident : $(;)? $stop:ident ( $ptr:ident ) = * $value:expr $(;)? ) => {
        $ptr.$stop($value, $mem)?;
    };
    ($mem:ident : $(;)? $stop:ident ( $ptr:ident ) = $value:expr $(;)? ) => {
        $ptr.$stop(&($value), $mem)?;
    };
    ($mem:ident : $(;)? $stop:ident ( $($ptr:tt)* ) = * $value:expr $(;)? ) => {
        $crate::Ptr!($($ptr)*).$stop($value, $mem)?;
    };
    ($mem:ident : $(;)? $stop:ident ( $($ptr:tt)* ) = $value:expr $(;)? ) => {
        $crate::Ptr!($($ptr)*).$stop($value, $mem)?;
    };


    ($mem:ident : $(;)? * $ptr:ident = * $value:expr $(; $($rest:tt)* )? ) => {
        $ptr.store($value, $mem)?;
        $( $crate::mem!($mem : $($rest)*); )?
    };
    ($mem:ident : $(;)? * $ptr:ident = $value:expr $(; $($rest:tt)* )? ) => {
         $ptr.store(&($value), $mem)?;
         $( $crate::mem!($mem : $($rest)*); )?
    };
    ($mem:ident : $(;)? * ($($ptr:tt)*) = * $value:expr $(; $($rest:tt)* )? ) => {
        $crate::Ptr!($($ptr)*).store($value, $mem)?;
        $( $crate::mem!($mem : $($rest)*); )?
    };
    ($mem:ident : $(;)? * ($($ptr:tt)*) = $value:expr $(; $($rest:tt)* )? ) => {
         $crate::Ptr!($($ptr)*).store(&($value), $mem)?;
         $( $crate::mem!($mem : $($rest)*); )?
    };
    ($mem:ident : $(;)? $stop:ident ( $ptr:ident ) = * $value:expr $(; $($rest:tt)* )? )=> {
        $ptr.$stop($value, $mem)?;
        $( $crate::mem!($mem : $($rest)*); )?
    };
    ($mem:ident : $(;)? $stop:ident ( $ptr:ident ) = $value:expr $(; $($rest:tt)* )? )=> {
        $ptr.$stop(&($value), $mem)?;
         $( $crate::mem!($mem : $($rest)*); )?
    };
    ($mem:ident : $(;)? $stop:ident ( $($ptr:tt)* ) = * $value:expr $(; $($rest:tt)* )? ) => {
        $crate::Ptr!($($ptr)*).$stop($value, $mem)?;
        $( $crate::mem!($mem : $($rest)*); )?
    };
    ($mem:ident : $(;)? $stop:ident ( $($ptr:tt)* ) = $value:expr $(; $($rest:tt)* )? ) => {
        $crate::Ptr!($($ptr)*).$stop($value, $mem)?;
         $( $crate::mem!($mem : $($rest)*); )?
    };
    (($mem:expr) : $( $rest:tt )* ) => {
        let mem = { $mem };
         $crate::mem!(mem : $($rest)*);
    };
}
