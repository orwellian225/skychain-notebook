use core::fmt::Debug;
use toml;

pub mod types;

#[typetag::serde(tag = "type")]
pub trait Cell {
	/// Fetch the unique identifier associated with the cell
	/// * identifier is a hash rather than a number
	fn get_identifier(&self) -> u128;

	/// Convert the cell into TOML data
	fn get_cell_data(&self) -> Result<String, toml::ser::Error>;
}
impl Debug for dyn Cell {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Cell{{{},\n{:?}}}", self.get_identifier(), self.get_cell_data())
    }
}
