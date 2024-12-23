mod vscode;
mod suffix;

pub trait Feature {
    // Returns a iterator of (schema_path, instance_path) pairs.
    fn get_instances(&self) -> impl Iterator<Item = (String, String)>;
}

// pub use vscode::Vscode;
pub use suffix::Suffix;
