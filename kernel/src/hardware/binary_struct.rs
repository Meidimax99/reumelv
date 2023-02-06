use core::{
    cmp::PartialOrd,
    ops::{BitAnd, BitOr, Range, Shl},
};
use riscv_utils::RegisterEntry;

/// Byte is a predefined type which is special case of the [`BinaryStruct`].
/// The special case is a binary struct with the generic T as [`u8`].
pub type Byte = BinaryStruct<u8>;

/// The binary struct enables operations on a binary datatype T.
/// Most of the operations must be implemented by T.
/// Therefore, the main purpose is the readability of those binary operations.
///
/// To use the binary struct the given type T must implement traits for:
///     - left shifts
///     - bit operations for logical 'and' and 'or'
///     - partial order
///     - copy
///     - iterators
///
/// Obviously, the binary struct derives the duplication traits [`clone`] and [`copy`] from the given type T, to enable those for the binary struct itself.
///
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BinaryStruct<T>(T);
impl<T> BinaryStruct<T>
where
    T: BinaryOperations
        + Shl<Output = T>
        + PartialOrd
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + Copy,
    Range<T>: Iterator<Item = T>,
{
    /// Checks for a bit value of the binary struct at the given location [bit].
    ///
    /// There are three possibilities of how the method behaves:
    /// - the value for  [bit] is 1, the true is returned
    /// - the value for [bit] is 0, the false is returned
    /// - the [bit] is out of the scope for the binary struct T, the false is returned
    ///
    /// For example:
    /// let BinaryStruct byte = BinaryStruct<u8>;
    /// let bit_position = 0;
    /// if (byte.is_set(bit_position)){
    ///     print("Bit is set to 1 at bit_position.")
    /// } else {
    ///     print("Bit is set to 0 at bit_position.")
    /// }
    ///
    ///
    pub fn is_set(&self, bit: usize) -> bool {
        if let Some(bit) = Self::transform_bit(bit) {
            return self.0 & T::one() << bit != T::zero();
        }
        return false;
    }

    /// Sets the bit at the position [bit] depending on the value [set].
    /// If [set] is:
    /// - false, the bit will be set to 0
    /// - true, the bit will be set to 1
    ///
    /// If the position [bit] is out of scope of the type T, the called object will not change.
    ///
    /// For example:
    /// let BinaryStruct byte = BinaryStruct<u8>;
    ///
    /// byte.at(4, true);
    /// print("The byte has set the 4 bit to 1");
    /// print(byte_is_set(4));
    /// byte.at(4, false);
    /// print("The byte has set the 4 bit to 0");
    ///
    pub fn at(&mut self, bit: usize, set: bool) {
        let bit = match Self::transform_bit(bit) {
            Some(bit) => bit,
            None => return,
        };
        if set {
            self.0 = self.0 | T::one() << bit;
        } else {
            self.0 = self.0 & (T::one() << bit).inverse();
        }
    }

    /// Sets a bit based on the [register_entry].
    /// Thereby, the [register_entry] holds the bit_position and the boolean value of the bit to be set.
    ///
    /// See also [`RegisterEntry`]
    pub fn write_register_entry(&mut self, register_entry: RegisterEntry) {
        let (bit, set) = register_entry;
        self.at(bit, set)
    }

    /// Returns a readable reference of it self.
    /// This does not borrow the ownership!
    pub fn get(&self) -> T {
        self.0
    }

    /// Returns an optional of a [`BinaryStruct`], holds the value of [bit].
    /// The type of the BinaryStruct is T (the type of the object this method was called on).
    /// The main use of this method is to change the type of [bit] from [`usize`] to BinaryStruct of the type T.
    ///  
    /// "None" will be returned, if size of the BinaryStruct T is smaller than the value [bit].
    /// This means the value of [bit] is to large to be saved in the binary size of T.
    ///
    fn transform_bit(bit: usize) -> Option<T> {
        if bit >= T::bit_size() {
            return None;
        }
        Some(T::from(bit))
    }
}

impl<T> From<T> for BinaryStruct<T> {
    fn from(data: T) -> Self {
        BinaryStruct(data)
    }
}

/// A trait to perform binary operations on the given type.
/// The type needs to implement the following functions with this desired outcome:
/// - bit_size -> the amount of bits for the given type
/// - one -> returns 1 in its type
/// - zero -> returns 0 in its type
/// - 10 -> returns 10 in its type
/// - inverse -> inverses the current value with the "!" operation
/// - from(data: usize) -> builds the type from the given element
/// - into_u8 -> returns an u8 interpretation of it self -> be aware: some bits might be cut off!
pub trait BinaryOperations {
    fn bit_size() -> usize;
    fn one() -> Self;
    fn zero() -> Self;
    fn ten() -> Self;
    fn inverse(self) -> Self;
    fn from(data: usize) -> Self;
    fn into_u8(self) -> u8;
}

/// Types implementing the MaxDigits trait need to implement a function max_digits().
/// This max_digits function returns a byte slice with the length of max_digits bytes.
/// The length is staticial given during the implementation of the trait for the specific type.
pub trait MaxDigits<const DIGITS: usize> {
    fn max_digits() -> [u8; DIGITS];
}
impl MaxDigits<20> for usize {
    fn max_digits() -> [u8; 20] {
        [0; 20]
    }
}

binary_operations!(usize, u8, u32, u64);
/// A simple macro to create [`BinaryOperations`] for numerical primitive types. Other types are not supported but maybe added manually.
macro_rules! binary_operations {
    ($($type:ty),+) => {
        $(impl BinaryOperations for $type {
            fn bit_size() -> usize {
                <$type>::BITS as usize
            }
            fn one() -> Self {
                1
            }
            fn zero() -> Self {
                0
            }
            fn inverse(self) -> Self {
                !self
            }

            fn from(data: usize) -> Self {
                data as Self
            }

            fn ten() -> Self {
                10
            }

            fn into_u8(self) -> u8 {
                self as u8
            }
        })+
    };
}
use binary_operations;
