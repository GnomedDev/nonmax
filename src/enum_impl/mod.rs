#[cfg(feature = "enum_repr_16")]
pub(crate) mod i16_repr;
pub(crate) mod i8_repr;
pub(crate) mod u8_repr;

macro_rules! nonmax {
    ( $nonmax: ident, $primitive: ident, $byte_repr: ident ) => {
        /// An integer that is known not to equal its maximum value.
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $nonmax($byte_repr);

        impl $nonmax {
            /// Creates a new non-max if the given value is not the maximum
            /// value.
            pub const fn new(value: $primitive) -> Option<Self> {
                match $byte_repr::new(value) {
                    Some(byte) => Some(Self(byte)),
                    None => None,
                }
            }

            /// Creates a new non-max without checking the value.
            ///
            /// # Safety
            ///
            /// The value must not equal the maximum representable value for the
            /// primitive type.
            #[inline]
            pub const unsafe fn new_unchecked(value: $primitive) -> Self {
                match Self::new(value) {
                    Some(this) => this,
                    None => unsafe { std::hint::unreachable_unchecked() },
                }
            }

            /// Returns the value as a primitive type.
            #[inline]
            pub const fn get(&self) -> $primitive {
                self.0 as u8 as $primitive
            }
        }
    };
}

pub(crate) use nonmax;
