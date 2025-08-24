use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::Cell;

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