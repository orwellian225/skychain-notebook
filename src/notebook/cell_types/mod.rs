use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum CellType {
    MarkdownCell,
    KernelCell { kernel_identifier: String }
}