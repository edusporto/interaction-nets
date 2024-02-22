use std::collections::HashMap;

use slab::Slab;

/// Key for Slabs
type Key = usize;

/// Represents a port in the Interaction Net
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct PortKey(Key);

/// Represents a cell in the Interaction Net
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct CellKey(Key);

/// Interaction Combinator
#[derive(Clone, Copy)]
pub struct Cell {
    pub(crate) primary: PortKey,
    pub(crate) cell_type: CellType,
}

/// Types of Interaction Combinators
#[derive(Clone, Copy)]
pub enum CellType {
    /// Eraser
    Era,
    /// Constructor
    Con(PortKey, PortKey),
    /// Duplicator
    Dup(PortKey, PortKey),
}

/// Ports can be free or part of a cell. They are connected to
/// each other by wires in the interaction net.
#[derive(Clone, Copy)]
pub struct Port {
    pub(crate) cell: Option<CellKey>,
}

/// Interaction Net
///
/// Contains interaction combinators, the ports that connect them, and
/// the wires between the ports.
pub struct InteractionNet {
    ports: Slab<Port>,
    cells: Slab<Cell>,
    wires: HashMap<PortKey, PortKey>,
}

impl InteractionNet {
    pub fn new() -> Self {
        Self {
            ports: Slab::new(),
            cells: Slab::new(),
            wires: HashMap::new(),
        }
    }

    pub fn create_port(&mut self) -> PortKey {
        let port = Port { cell: None };
        PortKey(self.ports.insert(port))
    }

    pub fn remove_port(&mut self, port_key: PortKey) {
        if self.ports[port_key.0].cell.is_some() || self.wires.contains_key(&port_key) {
            panic!("can only remove unconnected free ports");
        }

        self.ports.remove(port_key.0);
    }

    pub fn connect_ports(&mut self, p1: PortKey, p2: PortKey) {
        if p1 == p2 {
            panic!("cannot connect port to itself");
        }

        self.wires.insert(p1, p2);
        self.wires.insert(p2, p1);
    }

    pub fn disconnect_ports(&mut self, p1: PortKey, p2: PortKey) {
        if self.wires.get(&p1) == Some(&p2) {
            self.wires.remove(&p1);
            self.wires.remove(&p2);
        }
    }

    pub fn get_port(&self, port_key: PortKey) -> &Port {
        &self.ports[port_key.0]
    }

    pub fn ports_connected(&self, port_key1: PortKey, port_key2: PortKey) -> bool {
        self.wires.get(&port_key1).map(|port| *port == port_key2) == Some(true)
    }

    pub fn insert_cell(&mut self, cell: Cell) -> CellKey {
        let cell_key = CellKey(self.cells.insert(cell));
        self.ports[cell.primary.0].cell = Some(cell_key);
        match cell.cell_type {
            CellType::Con(p1, p2) => {
                self.ports[p1.0].cell = Some(cell_key);
                self.ports[p2.0].cell = Some(cell_key);
            }
            CellType::Dup(p1, p2) => {
                self.ports[p1.0].cell = Some(cell_key);
                self.ports[p2.0].cell = Some(cell_key);
            }
            CellType::Era => (),
        }

        cell_key
    }

    pub fn remove_cell(&mut self, cell_key: CellKey) {
        let cell = self.cells[cell_key.0];
        self.ports[cell.primary.0].cell = None;
        match cell.cell_type {
            CellType::Con(p1, p2) => {
                self.ports[p1.0].cell = None;
                self.ports[p2.0].cell = None;
            }
            CellType::Dup(p1, p2) => {
                self.ports[p1.0].cell = None;
                self.ports[p2.0].cell = None;
            }
            CellType::Era => (),
        }

        self.cells.remove(cell_key.0);
    }

    pub fn get_cell(&self, cell_key: CellKey) -> &Cell {
        &self.cells[cell_key.0]
    }

    pub fn get_cells(&self) -> impl Iterator<Item = (CellKey, &Cell)> {
        self.cells.iter().map(|(key, cell)| (CellKey(key), cell))
    }
}

impl Default for InteractionNet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use slab::Slab;

    #[test]
    fn test() {
        let mut slab = Slab::new();
        slab.insert(1);
        slab.insert(2);
        slab.insert(1);

        println!("{slab:?}");
        println!("{:?}", slab.iter().collect::<Vec<_>>());
    }
}
