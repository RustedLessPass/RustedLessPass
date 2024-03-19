#![no_std]
#![allow(non_upper_case_globals)]

#[cfg(feature = "std")]
extern crate std;

use core::mem::MaybeUninit;

use pbkdf2::{
    hmac::{Hmac, Mac as _},
    pbkdf2_hmac,
};
use sha2::{Sha256, Sha384, Sha512};

/// Selects the hash algorithm to use in PBKDF2.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Algorithm {
    /// SHA2-256.
    ///
    /// This is the algorithm used by the canonical LessPass implementation.
    SHA256,

    /// SHA2-384.
    ///
    /// Note: Using this algorithm makes the generated passwords different from
    /// every other LessPass implementation.
    SHA384,

    /// SHA2-512.
    ///
    /// Note: Using this algorithm makes the generated passwords different from
    /// every other LessPass implementation.
    SHA512,
}

bitflags::bitflags! {
    /// Flag that describes what characters are allowed when generating a
    /// password.
    #[derive(Clone, Copy)]
    #[repr(transparent)]
    pub struct CharacterSet: u8 {
        const Uppercase = 0b0001;
        const Lowercase = 0b0010;
        const Numbers   = 0b0100;
        const Symbols   = 0b1000;

        const Letters   = Self::Uppercase.bits() | Self::Lowercase.bits();
        const All       = Self::Letters.bits() | Self::Numbers.bits() | Self::Symbols.bits();
    }
}

impl CharacterSet {
    const LOWERCASE: &'static str = "abcdefghijklmnopqrstuvwxyz";
    const UPPERCASE: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMBERS: &'static str = "0123456789";
    const SYMBOLS: &'static str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

    /// Returns a string that contains all the characters that may be used to
    /// generate a password.
    pub const fn get_characters(self) -> &'static str {
        match (self.contains(Self::Lowercase), self.contains(Self::Uppercase), self.contains(Self::Numbers), self.contains(Self::Symbols)) {
            (true , true , true , true ) => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~",
            (true , true , true , false) => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
            (true , true , false, true ) => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~",
            (true , true , false, false) => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",

            (true , false, true , true ) => "abcdefghijklmnopqrstuvwxyz0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~",
            (true , false, true , false) => "abcdefghijklmnopqrstuvwxyz0123456789",
            (true , false, false, true ) => "abcdefghijklmnopqrstuvwxyz!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~",
            (true , false, false, false) => Self::LOWERCASE,

            (false, true , true , true ) => "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~",
            (false, true , true , false) => "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
            (false, true , false, true ) => "ABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~",
            (false, true , false, false) => Self::UPPERCASE,

            (false, false, true , true ) => "0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~",
            (false, false, true , false) => Self::NUMBERS,
            (false, false, false, true ) => Self::SYMBOLS,

            _ => ""
        }
    }

    /// Returns a list of all the sets of characters that may be used to
    /// generate a password.
    ///
    /// The second item of the tuple corresponds to the length of the list;
    /// items greater than that length are simply the empty string `""`.
    pub const fn get_sets(self) -> ([&'static str; 4], usize) {
        let mut sets = [""; 4];
        let mut sets_len = 0;

        if self.contains(Self::Lowercase) {
            sets[sets_len] = Self::LOWERCASE;
            sets_len += 1;
        }
        if self.contains(Self::Uppercase) {
            sets[sets_len] = Self::UPPERCASE;
            sets_len += 1;
        }
        if self.contains(Self::Numbers) {
            sets[sets_len] = Self::NUMBERS;
            sets_len += 1;
        }
        if self.contains(Self::Symbols) {
            sets[sets_len] = Self::SYMBOLS;
            sets_len += 1;
        }

        (sets, sets_len)
    }
}

/// Generates the salt needed to compute the entropy using a combinaison of the
/// target website, login and counters, and writes it to `output`.
///
/// Returns `Ok(written_size)` if `output` is large enough, and
/// `Err(required_size)` if it isn't (in which case nothing will be written).
#[inline]
pub fn generate_salt_to(
    website: &str,
    username: &str,
    counter: u32,
    output: &mut [u8],
) -> Result<usize, usize> {
    generate_salt_to_uninit(
        website,
        username,
        counter,
        slice_to_maybe_uninit_mut(output),
    )
    .map(|x| x.len())
}

/// Same as [`generate_salt_to`], but works with an uninitialized output
/// buffer, which is okay since it only writes to it, without reading from it.
pub fn generate_salt_to_uninit<'out>(
    website: &str,
    username: &str,
    counter: u32,
    output: &'out mut [MaybeUninit<u8>],
) -> Result<&'out mut [u8], usize> {
    let mut counter_buf = [MaybeUninit::uninit(); 8];
    let counter = {
        let mut counter = counter as usize;
        let mut i = counter_buf.len();

        while counter != 0 {
            counter_buf[i - 1].write(b"0123456789abcdef"[counter & 0xf]);
            counter >>= 4;
            i -= 1;
        }

        &counter_buf[i..]
    };

    let required_len = website.len() + username.len() + counter.len();
    let mut offset = 0;

    if output.len() < required_len {
        return Err(required_len);
    }

    output[offset..offset + website.len()]
        .copy_from_slice(slice_to_maybe_uninit_ref(website.as_bytes()));
    offset += website.len();

    output[offset..offset + username.len()]
        .copy_from_slice(slice_to_maybe_uninit_ref(username.as_bytes()));
    offset += username.len();

    output[offset..offset + counter.len()].copy_from_slice(counter);
    offset += counter.len();

    // SAFETY: all bytes up to `offset` were written to (and thus initialized).
    Ok(unsafe { &mut *(&mut output[..offset] as *mut [MaybeUninit<u8>] as *mut [u8]) })
}

/// Same as [`generate_salt_to`], but directly returns the salt instead of
/// requiring a mutable output buffer.
#[cfg(feature = "std")]
#[inline]
pub fn generate_salt(website: &str, username: &str, counter: u32) -> std::vec::Vec<u8> {
    let mut counter_copy = counter;
    let mut counter_len = 0;

    while counter_copy != 0 {
        counter_copy >>= 4;
        counter_len += 1;
    }

    let mut uninit_output = uninit_vec(website.len() + username.len() + counter_len);
    let result = generate_salt_to_uninit(website, username, counter, &mut uninit_output);

    // Make sure that the result (the initialized subslice of `uninit_output`)
    // has the same length as `uninit_output` itself.
    debug_assert_eq!(result.map(|x| x.len()), Ok(uninit_output.len()));

    // SAFETY: `uninit_output` was fully initialized in
    // `generate_salt_to_uninit`.
    unsafe { assume_init_vec(uninit_output) }
}

/// The minimum length of the entropy in bytes, inclusive.
pub const MIN_ENTROPY_LEN: usize = 1;

/// The maximum length of the entropy in bytes, inclusive.
pub const MAX_ENTROPY_LEN: usize = 64;

/// Generates the entropy needed to render the end password using a previously
/// computed salt and a master password, and writes it to `output`.
///
/// Note that this function does not support an `uninit` output buffer since it
/// internally uses external functions that only accept initialized data.
///
/// # Panics
///
/// Panics if `output` is smaller than [`MIN_ENTROPY_LEN`] or greater than
/// [`MAX_ENTROPY_LEN`], or if `master_password` is empty, or if `iterations` is
/// 0, or if `salt` is empty.
pub fn generate_entropy_to(
    master_password: &str,
    salt: &[u8],
    algorithm: Algorithm,
    iterations: u32,
    output: &mut [u8],
) {
    assert!(!master_password.is_empty());
    assert!(!salt.is_empty());
    assert!(iterations > 0);
    assert!((MIN_ENTROPY_LEN..=MAX_ENTROPY_LEN).contains(&output.len()));

    match algorithm {
        Algorithm::SHA256 => {
            pbkdf2_hmac::<Sha256>(master_password.as_bytes(), salt, iterations, output)
        }
        Algorithm::SHA384 => {
            pbkdf2_hmac::<Sha384>(master_password.as_bytes(), salt, iterations, output)
        }
        Algorithm::SHA512 => {
            pbkdf2_hmac::<Sha512>(master_password.as_bytes(), salt, iterations, output)
        }
    }
}

/// Same as [`generate_entropy_to`], but directly returns the entropy buffer
/// instead of requiring a mutable output buffer.
///
/// The length of the resulting buffer is:
/// - 32 bytes for SHA-256,
/// - 48 bytes for SHA-384,
/// - 64 bytes for SHA-512.
#[cfg(feature = "std")]
#[inline]
pub fn generate_entropy(
    master_password: &str,
    salt: &[u8],
    algorithm: Algorithm,
    iterations: u32,
) -> std::vec::Vec<u8> {
    let out_len = match algorithm {
        Algorithm::SHA256 => 256 / 8,
        Algorithm::SHA384 => 384 / 8,
        Algorithm::SHA512 => 512 / 8,
    };
    let mut out = std::vec![0; out_len];

    generate_entropy_to(master_password, salt, algorithm, iterations, &mut out);

    out
}

// Wrap type definition in a private module to use `#[allow(...)]`.
#[allow(clippy::all)]
mod private {
    uint::construct_uint! {
        /// A 512-bits integer.
        pub(super) struct BigUint(8 /* 64-bit words */);
    }
}

use self::private::BigUint;

/// The minimum length of the rendered password, inclusive.
pub const MIN_PASSWORD_LEN: usize = 5;

/// The maximum length of the rendered password, inclusive.
pub const MAX_PASSWORD_LEN: usize = 35;

/// Same as [`render_password_to`], but works with an uninitialized output
/// buffer, which is okay since it only writes to it, without reading from it.
pub fn render_password_to_uninit<'out>(
    entropy: &[u8],
    charset: CharacterSet,
    output: &'out mut [MaybeUninit<u8>],
) -> &'out mut [u8] {
    assert!(!entropy.is_empty());
    assert!(!charset.is_empty());

    let len = output.len();

    assert!((MIN_PASSWORD_LEN..=MAX_PASSWORD_LEN).contains(&len));

    let chars = charset.get_characters().as_bytes();
    let (sets, sets_len) = charset.get_sets();

    let mut offset = 0;

    // Generate initial part of the password.
    let mut quotient = BigUint::from_big_endian(entropy);

    for _ in 0..(len - sets_len) {
        let rem = div_rem(&mut quotient, chars.len());

        output[offset].write(chars[rem]);
        offset += 1;
    }

    // Compute some random characters in each set in order to ensure all sets
    // will be used at least once.
    let mut additional_chars = [0; 4];
    let mut additional_chars_len = 0;

    for set in sets.into_iter().take(sets_len) {
        let rem = div_rem(&mut quotient, set.len());

        additional_chars[additional_chars_len] += set.as_bytes()[rem];
        additional_chars_len += 1;
    }

    // Finalize last part of password using previously generated characters.
    for ch in additional_chars.into_iter().take(additional_chars_len) {
        let rem = div_rem(&mut quotient, offset);

        // Insert `ch` at `rem`.
        output.copy_within(rem..output.len() - 1, rem + 1);
        output[rem].write(ch);

        offset += 1;
    }

    debug_assert_eq!(offset, len);

    // SAFETY: all bytes in `output` were written to (`offset == len`).
    unsafe { &mut *(output as *mut [MaybeUninit<u8>] as *mut [u8]) }
}

/// Generates a password of the given length using the provided entropy and
/// character sets, and writes it to `output`.
///
/// # Panics
///
/// Panics if `output` is smaller than [`MIN_PASSWORD_LEN`] or greater than
/// [`MAX_PASSWORD_LEN`], or if `entropy` is empty, or if `charset` is empty.
#[inline]
pub fn render_password_to(entropy: &[u8], charset: CharacterSet, output: &mut [u8]) {
    render_password_to_uninit(entropy, charset, slice_to_maybe_uninit_mut(output));
}

/// Same as [`render_password_to`], but directly returns the rendered password
/// instead of requiring a mutable output buffer.
#[cfg(feature = "std")]
#[inline]
pub fn render_password(entropy: &[u8], charset: CharacterSet, len: usize) -> std::string::String {
    let mut uninit_output = uninit_vec(len);

    render_password_to_uninit(entropy, charset, &mut uninit_output);

    // SAFETY: `uninit_output` was fully initialized in
    // `render_password_to_uninit`.
    let output = unsafe { assume_init_vec(uninit_output) };

    // SAFETY: characters are all extracted from `charset`, which only contains
    // a limited set of ASCII characters.
    unsafe { std::string::String::from_utf8_unchecked(output) }
}

/// Return the SHA-256 fingerprint that corresponds to the given master password.
pub fn get_fingerprint(password: &str) -> [u8; 32] {
    let mut mac = Hmac::<Sha256>::new_from_slice(password.as_bytes())
        .expect("Hmac's new_from_slice implementation is infallible");
    mac.update(b"");

    // SAFETY: `GenericArray<u8, N>` is equivalent to `[u8; N]`.
    unsafe { core::mem::transmute(mac.finalize().into_bytes()) }
}

/// Updates `quot` in place after dividing it by `div`, and returns the
/// remainder.
#[inline]
fn div_rem(quot: &mut BigUint, div: usize) -> usize {
    let (new_quot, rem) = quot.div_mod(div.into());

    *quot = new_quot;

    if cfg!(all(target_endian = "little", target_pointer_width = "64")) {
        // We can optimize the case where the low-end of the remainder is
        // directly equivalent to an `usize` value.
        rem.low_u64() as usize
    } else {
        // We use `as_usize` below, and given that we divided by an `usize`
        // above it is certain that `as_usize` will succeed.
        rem.as_usize()
    }
}

#[inline(always)]
fn slice_to_maybe_uninit_ref<T>(slice: &[T]) -> &[MaybeUninit<T>] {
    // SAFETY: a `T` is just an initialized `MaybeUninit<T>`.
    unsafe { &*(slice as *const [T] as *const [MaybeUninit<T>]) }
}

#[inline(always)]
fn slice_to_maybe_uninit_mut<T>(slice: &mut [T]) -> &mut [MaybeUninit<T>] {
    // SAFETY: a `T` is just an initialized `MaybeUninit<T>`.
    unsafe { &mut *(slice as *mut [T] as *mut [MaybeUninit<T>]) }
}

#[cfg(feature = "std")]
#[inline(always)]
fn uninit_vec<T>(len: usize) -> std::vec::Vec<MaybeUninit<T>> {
    // Rust can optimize this well in theory:
    // https://rust.godbolt.org/z/fr3cKbd9s.
    // Since this is only use in `std` builds and that it requires an allocation
    // anyway, the lack of guarantee that this is as optimized as using
    // `set_len` is acceptable.
    std::iter::repeat_with(MaybeUninit::uninit)
        .take(len)
        .collect()
}

#[cfg(feature = "std")]
#[inline(always)]
unsafe fn assume_init_vec(vec: std::vec::Vec<MaybeUninit<u8>>) -> std::vec::Vec<u8> {
    let mut vec = core::mem::ManuallyDrop::new(vec);
    let (ptr, len, capacity) = (vec.as_mut_ptr(), vec.len(), vec.capacity());

    std::vec::Vec::from_raw_parts(ptr.cast(), len, capacity)
}

#[cfg(test)]
mod fingerprint_tests {
    use super::*;

    #[test]
    fn empty() {
        // For keys with messages smaller than SHA256's block size (64
        // bytes), the key is padded with zeros.
        assert_eq!(
            &get_fingerprint("")[..],
            &[
                182, 19, 103, 154, 8, 20, 217, 236, 119, 47, 149, 215, 120, 195, 95, 197, 255, 22,
                151, 196, 147, 113, 86, 83, 198, 199, 18, 20, 66, 146, 197, 173
            ]
        );
    }

    #[test]
    fn small() {
        // Same as `empty`.
        assert_eq!(
            &get_fingerprint("foo")[..],
            &[
                104, 55, 22, 217, 215, 248, 46, 237, 23, 76, 108, 174, 190, 8, 110, 233, 51, 118,
                199, 157, 124, 97, 221, 103, 14, 160, 15, 127, 141, 110, 176, 168
            ]
        );
    }

    #[test]
    fn same_as_block_size() {
        // If it matches the block size, it is used as-is.
        assert_eq!(
            &get_fingerprint("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef")[..],
            &[
                8, 18, 71, 220, 104, 187, 127, 175, 191, 19, 34, 0, 19, 160, 171, 113, 219, 139,
                98, 141, 103, 145, 97, 248, 123, 94, 91, 217, 225, 155, 20, 148
            ]
        );
    }

    #[test]
    fn larger_than_block_size() {
        // If it is larger, it is hashed first.
        assert_eq!(
            &get_fingerprint(
                "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdeflarger than SHA256's block size"
            )[..],
            &[
                46, 55, 32, 12, 232, 162, 61, 209, 182, 227, 200, 183, 211, 185, 6, 171, 72, 182,
                239, 151, 196, 213, 132, 130, 106, 95, 106, 71, 156, 0, 103, 234
            ]
        );
    }
}

#[cfg(all(test, feature = "std"))]
mod entropy_tests {
    use super::*;

    /// Transforms a string like `"abcd"` into a buffer like `b"\xAB\xCD"`.
    fn to_bytes(s: &str) -> std::vec::Vec<u8> {
        let len = s.len() / 2;
        let mut result = std::vec::Vec::with_capacity(len);

        for i in 0..len {
            result.push(u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap());
        }

        result
    }

    #[test]
    fn defaults() {
        // https://github.com/lesspass/lesspass/blob/bab758c12b565120d9e6a5ff8a395ae1f3d69dbb/packages/lesspass-entropy/test/index.test.js#L5-L17
        let salt = generate_salt("example.org", "contact@example.org", 1);
        let entropy = generate_entropy("password", &salt, Algorithm::SHA256, 100_000);

        assert_eq!(
            entropy,
            to_bytes("dc33d431bce2b01182c613382483ccdb0e2f66482cbba5e9d07dab34acc7eb1e"),
        );
    }

    #[test]
    fn unicode() {
        // https://github.com/lesspass/lesspass/blob/bab758c12b565120d9e6a5ff8a395ae1f3d69dbb/packages/lesspass-entropy/test/index.test.js#L40-L61
        let salt = generate_salt("example.org", "❤", 1);
        let entropy = generate_entropy("I ❤ LessPass", &salt, Algorithm::SHA256, 100_000);

        assert_eq!(
            entropy,
            to_bytes("4e66cab40690c01af55efd595f5963cc953d7e10273c01827881ebf8990c627f"),
        );
    }

    #[test]
    fn sha512() {
        // https://github.com/lesspass/lesspass/blob/bab758c12b565120d9e6a5ff8a395ae1f3d69dbb/packages/lesspass-entropy/test/index.test.js#L62-L80
        let salt = generate_salt("example.org", "contact@example.org", 1);
        let mut entropy = [0; 16];
        generate_entropy_to("password", &salt, Algorithm::SHA512, 8192, &mut entropy);

        assert_eq!(&entropy[..], to_bytes("fff211c16a4e776b3574c6a5c91fd252"),);
    }
}
