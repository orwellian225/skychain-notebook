use core::fmt::Debug;

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use toml;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct MarkdownCell {
	identifier: Uuid,
	content: String
}

impl MarkdownCell {
	pub fn create_cell(markdown_content: String) -> MarkdownCell {
		MarkdownCell {
			identifier: Uuid::new_v4(),
			content: markdown_content
		}
	}
}

#[typetag::serde]
impl Cell for MarkdownCell {
	fn get_identifier(&self) -> u128 { self.identifier.as_u128() }
	fn get_cell_data(&self) -> Result<String, toml::ser::Error> { toml::to_string_pretty(self) }
}
