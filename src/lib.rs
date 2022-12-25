use std::hash::Hasher;

use pyo3::prelude::*;

mod numeric_finish;

use numeric_finish::{Hash128, NumericFinish};

#[pyclass]
/// An implementation of SipHash 1-3 with 64-bit output.
struct SipHash13(siphasher::sip::SipHasher13);
#[pyclass]
/// An implementation of SipHash 2-4 with 64-bit output.
struct SipHash24(siphasher::sip::SipHasher24);
#[pyclass]
/// An implementation of SipHash 1-3 with 128-bit output.
struct SipHash13_128(siphasher::sip128::SipHasher13);
#[pyclass]
/// An implementation of SipHash 2-4 with 128-bit output.
struct SipHash24_128(siphasher::sip128::SipHasher24);

macro_rules! siphash_impl {
    ($rust_name:ty, $impl_name:ty, $digest_size:expr) => {
        #[pymethods]
        impl $rust_name {
            #[new]
            #[args(key0 = "0", key1 = "0")]
            fn new(key0: u64, key1: u64) -> Self {
                Self(<$impl_name>::new_with_keys(key0, key1))
            }

            fn update(&mut self, data: &[u8]) {
                self.0.write(data)
            }

            #[getter]
            fn digest_size(&self) -> usize {
                $digest_size
            }

            /// Return the digest value as a bytes object. (Big endian of intdigest)
            fn digest(&self) -> [u8; $digest_size] {
                self.intdigest().to_be_bytes()
            }

            /// Return the digest value as a string of hexadecimal digits. (Big endian of intdigest)
            fn hexdigest(&self) -> String {
                let width = $digest_size * 2;
                format!("{:0width$x}", self.intdigest().to_be())
            }

            /// Returns a single $digest_size bytes integer in native endain.
            fn intdigest(&self) -> <$rust_name as NumericFinish>::SingleOutput {
                self.finish_numeric_single()
            }

            /// Returns a structured intger output when applicable.
            fn intdigest_structured(&self) -> <$rust_name as NumericFinish>::Output {
                self.finish_numeric()
            }
        }
    };
}

siphash_impl!(SipHash13, siphasher::sip::SipHasher13, 8);
siphash_impl!(SipHash24, siphasher::sip::SipHasher24, 8);
siphash_impl!(SipHash13_128, siphasher::sip128::SipHasher13, 16);
siphash_impl!(SipHash24_128, siphasher::sip128::SipHasher24, 16);

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "siphasher")]
fn pysiphasher(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SipHash13>()?;
    m.add_class::<SipHash24>()?;
    m.add_class::<SipHash13_128>()?;
    m.add_class::<SipHash24_128>()?;
    m.add_class::<Hash128>()?;
    Ok(())
}
