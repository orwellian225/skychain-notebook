use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum CellType {
    MarkdownCell,
    KernelCell { kernel_identifier: String }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cell {
    identifier: u64,
    collapsed: bool,
    cell_type: CellType,
    content: String
}

impl Cell {
    pub fn create_cell(cell_identifier: u64, cell_type: CellType, cell_content: String) -> Cell {
        return Cell {
            identifier: cell_identifier,
            cell_type: cell_type,
            content: cell_content,
            collapsed: false
        }
    }
}