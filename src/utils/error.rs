#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BtreeError {
    LowerBoundTooLow,
    LowerBoundTooLarge,
}

impl std::error::Error for BtreeError {}

impl std::fmt::Display for BtreeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            BtreeError::LowerBoundTooLow => {
                "Lower Bound (lowest number of keys in a node) needs to be at least 2"
            }

            BtreeError::LowerBoundTooLarge => {
                "Lower Bound (lowest number of keys in a node) too large"
            }
        };

        write!(f, "{message}")
    }
}