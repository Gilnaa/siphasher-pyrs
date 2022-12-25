use super::*;
use siphasher::sip128::Hasher128;

#[pyclass]
pub struct Hash128 {
    #[pyo3(get)]
    low: u64,
    #[pyo3(get)]
    high: u64,
}

#[pymethods]
impl Hash128 {
    fn __repr__(&self) -> String {
        format!(
            "<Hash128 low={low} high={high}>",
            low = self.low,
            high = self.high
        )
    }
}

/// A trait for determining how hash is finalized into an integer form
/// for Python consumption.
pub trait NumericFinish {
    /// The type of the output.
    type Output;
    /// The type of the output when finalizing into a single integer.
    type SingleOutput;

    /// Finalize the hash into a structured output.
    fn finish_numeric(&self) -> Self::Output;

    /// Finalize the hash into a single integer.
    fn finish_numeric_single(&self) -> Self::SingleOutput;
}

impl NumericFinish for SipHash13 {
    type Output = u64;
    type SingleOutput = u64;

    fn finish_numeric(&self) -> Self::Output {
        self.0.finish()
    }

    fn finish_numeric_single(&self) -> Self::SingleOutput {
        self.0.finish()
    }
}

impl NumericFinish for SipHash24 {
    type Output = u64;
    type SingleOutput = u64;

    fn finish_numeric(&self) -> Self::Output {
        self.0.finish()
    }

    fn finish_numeric_single(&self) -> Self::SingleOutput {
        self.0.finish()
    }
}

impl NumericFinish for SipHash13_128 {
    type Output = Hash128;
    type SingleOutput = u128;

    fn finish_numeric(&self) -> Self::Output {
        let siphasher::sip128::Hash128 { h1: low, h2: high } = self.0.finish128();
        Hash128 { low, high }
    }

    fn finish_numeric_single(&self) -> Self::SingleOutput {
        let siphasher::sip128::Hash128 {
            h1: lower,
            h2: upper,
        } = self.0.finish128();

        ((upper as u128) << 64) | (lower as u128)
    }
}

impl NumericFinish for SipHash24_128 {
    type Output = Hash128;
    type SingleOutput = u128;

    fn finish_numeric(&self) -> Self::Output {
        let siphasher::sip128::Hash128 { h1: low, h2: high } = self.0.finish128();
        Hash128 { low, high }
    }

    fn finish_numeric_single(&self) -> Self::SingleOutput {
        let siphasher::sip128::Hash128 {
            h1: lower,
            h2: upper,
        } = self.0.finish128();

        ((upper as u128) << 64) | (lower as u128)
    }
}
