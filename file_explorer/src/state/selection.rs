use std::collections::HashSet;

use crate::fs::entries::FileEntry;

#[derive(Clone, Copy, PartialEq)]
pub enum SortColumn {
    Name,
    Size,
    Type,
    Modified,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

pub struct SelectionState {
    pub selected_indices: HashSet<usize>,
    pub sort_column: SortColumn,
    pub sort_order: SortOrder,
}

impl SelectionState {
    pub fn new() -> Self {
        Self {
            selected_indices: HashSet::new(),
            sort_column: SortColumn::Name,
            sort_order: SortOrder::Ascending,
        }
    }

    pub fn select_single(&mut self, index: usize) {
        self.selected_indices.clear();
        self.selected_indices.insert(index);
    }

    pub fn toggle(&mut self, index: usize) {
        if !self.selected_indices.remove(&index) {
            self.selected_indices.insert(index);
        }
    }

    pub fn clear(&mut self) {
        self.selected_indices.clear();
    }

    pub fn toggle_sort(&mut self, column: SortColumn) {
        if self.sort_column == column {
            self.sort_order = match self.sort_order {
                SortOrder::Ascending => SortOrder::Descending,
                SortOrder::Descending => SortOrder::Ascending,
            };
        } else {
            self.sort_column = column;
            self.sort_order = SortOrder::Ascending;
        }
    }

    pub fn sort_entries(&self, entries: &mut [FileEntry]) {
        let order_mul = match self.sort_order {
            SortOrder::Ascending => std::cmp::Ordering::Less,
            SortOrder::Descending => std::cmp::Ordering::Greater,
        };

        entries.sort_by(|a, b| {
            // Directories always first
            let dir_cmp = b.is_dir.cmp(&a.is_dir);
            if dir_cmp != std::cmp::Ordering::Equal {
                return dir_cmp;
            }

            let cmp = match self.sort_column {
                SortColumn::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                SortColumn::Size => a.size.cmp(&b.size),
                SortColumn::Type => a.extension.cmp(&b.extension),
                SortColumn::Modified => a.modified.cmp(&b.modified),
            };

            if cmp == std::cmp::Ordering::Less {
                order_mul
            } else if cmp == std::cmp::Ordering::Greater {
                order_mul.reverse()
            } else {
                std::cmp::Ordering::Equal
            }
        });
    }
}
