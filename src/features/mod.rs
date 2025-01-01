mod vscode;
mod suffix;

/// A feature of scheval that is capable of finding JSON instances and their corresponding schemas.
pub trait Feature {
    /// Generate a list of JSON instances and their corresponding schemas.
    fn get_instances(&self) -> impl Iterator<Item = (String, String)>;
}

pub use vscode::Vscode;
pub use suffix::Suffix;
