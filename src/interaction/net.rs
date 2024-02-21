use crate::interaction::graph::Cell;

use super::graph::{CellKey, CellType, InteractionGraph};

/// Interaction net
pub struct Net {
    graph: InteractionGraph,
}

impl Net {
    pub fn normalize(&mut self) {
        while self.perform_any_interaction() {}
    }

    pub fn perform_any_interaction(&mut self) -> bool {
        let mut connected_cells = self
            .graph
            .get_cells()
            .map(|(cell_key, _)| {
                // Match cell with its primary connection port
                let cell = self.graph.get_cell(cell_key);
                (cell_key, self.graph.get_port(cell.primary).cell)
            })
            .filter_map(|(cell_key1, connected_cell)| {
                // Only include cells with another cell connected to their primary port
                connected_cell.map(|cell_key2| (cell_key1, cell_key2))
            });

        let interactable = connected_cells.find(|(c1, c2)| self.can_interact(*c1, *c2));

        if let Some((cell_key1, cell_key2)) = interactable {
            drop(connected_cells);
            self.interact(cell_key1, cell_key2);
            return true;
        }

        false
    }

    pub fn can_interact(&self, cell_key1: CellKey, cell_key2: CellKey) -> bool {
        let cell1 = *self.graph.get_cell(cell_key1);
        let cell2 = *self.graph.get_cell(cell_key2);

        self.graph.ports_connected(cell1.primary, cell2.primary)
    }

    pub fn try_interact(&mut self, cell_key1: CellKey, cell_key2: CellKey) -> bool {
        if self.can_interact(cell_key1, cell_key2) {
            self.interact(cell_key1, cell_key2);
        }
        true
    }

    fn interact(&mut self, cell_key1: CellKey, cell_key2: CellKey) {
        let cell1 = *self.graph.get_cell(cell_key1);
        let cell2 = *self.graph.get_cell(cell_key2);

        // if !self.graph.ports_connected(cell1.primary, cell2.primary) {
        //     return;
        // }

        self.graph.disconnect_ports(cell1.primary, cell2.primary);
        let primary1 = cell1.primary;
        let primary2 = cell2.primary;

        use CellType::*;
        match (cell1.cell_type, cell2.cell_type) {
            (Era, Era) => {
                self.graph.remove_cell(cell_key1);
                self.graph.remove_cell(cell_key2);
            }

            (Era, Con(left, right))
            | (Con(left, right), Era)
            | (Era, Dup(left, right))
            | (Dup(left, right), Era) => {
                self.graph.remove_cell(cell_key1);
                self.graph.remove_cell(cell_key2);

                let port1 = self.graph.create_port();
                self.graph.connect_ports(port1, left);
                self.graph.insert_cell(Cell {
                    primary: port1,
                    cell_type: Era,
                });

                let port2 = self.graph.create_port();
                self.graph.connect_ports(port2, right);
                self.graph.insert_cell(Cell {
                    primary: port2,
                    cell_type: Era,
                });
            }

            (Con(left1, right1), Con(left2, right2)) | (Dup(left1, right1), Dup(left2, right2)) => {
                self.graph.remove_cell(cell_key1);
                self.graph.remove_cell(cell_key2);

                self.graph.connect_ports(left1, left2);
                self.graph.connect_ports(right1, right2);
            }

            (Con(left1, right1), Dup(left2, right2)) | (Dup(left2, right2), Con(left1, right1)) => {
                self.graph.remove_cell(cell_key1);
                self.graph.remove_cell(cell_key2);

                let (upper_dup_left, upper_dup_right) =
                    (self.graph.create_port(), self.graph.create_port());
                let (upper_con_left, upper_con_right) =
                    (self.graph.create_port(), self.graph.create_port());
                let (lower_dup_left, lower_dup_right) =
                    (self.graph.create_port(), self.graph.create_port());
                let (lower_con_left, lower_con_right) =
                    (self.graph.create_port(), self.graph.create_port());

                let _upper_dup = self.graph.insert_cell(Cell {
                    primary: left1,
                    cell_type: Dup(upper_dup_left, upper_dup_right),
                });
                let _upper_con = self.graph.insert_cell(Cell {
                    primary: right2,
                    cell_type: Con(upper_con_left, upper_con_right),
                });
                let _lower_dup = self.graph.insert_cell(Cell {
                    primary: right1,
                    cell_type: Dup(lower_dup_left, lower_dup_right),
                });
                let _lower_con = self.graph.insert_cell(Cell {
                    primary: left2,
                    cell_type: Con(lower_con_left, lower_con_right),
                });

                self.graph.connect_ports(upper_dup_right, upper_con_left);
                self.graph.connect_ports(upper_dup_left, lower_con_left);
                self.graph.connect_ports(lower_dup_right, upper_con_right);
                self.graph.connect_ports(lower_dup_left, lower_con_right);
            }
        }

        self.graph.remove_port(primary1);
        self.graph.remove_port(primary2);
    }
}
