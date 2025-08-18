use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum CellType {
    MarkdownCell(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cell {
    identifier: u64,
    cell_type: CellType,
    collapsed: bool
}

impl Cell {
    pub fn create_cell(cell_identifier: u64, cell_type: CellType) -> Cell {
        return Cell {
            identifier: cell_identifier,
            cell_type: cell_type,
            collapsed: false
        }
    }
}