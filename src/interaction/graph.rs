use std::collections::HashMap;

use slab::Slab;

type Key = usize;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct PortKey(Key);
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct CellKey(Key);

/// Interaction Combinator
#[derive(Clone, Copy)]
pub struct Cell {
    pub(crate) primary: PortKey,
    pub(crate) cell_type: CellType,
}

#[derive(Clone, Copy)]
pub enum CellType {
    /// Eraser
    Era,
    /// Constructor
    Con(PortKey, PortKey),
    /// Duplicator
    Dup(PortKey, PortKey),
}

#[derive(Clone, Copy)]
pub struct Port {
    pub(crate) cell: Option<CellKey>,
}

pub struct InteractionGraph {
    ports: Slab<Port>,
    cells: Slab<Cell>,
    wires: HashMap<PortKey, PortKey>,
}

impl InteractionGraph {
    pub fn create_port(&mut self) -> PortKey {
        let port = Port { cell: None };
        let port_key = PortKey(self.ports.insert(port));
        port_key
    }

    pub fn remove_port(&mut self, port_key: PortKey) {
        if self.ports[port_key.0].cell.is_some() || self.wires.contains_key(&port_key) {
            panic!("can only remove unconnected free ports");
        }

        self.ports.remove(port_key.0);
    }

    pub fn connect_ports(&mut self, p1: PortKey, p2: PortKey) {
        self.wires.insert(p1, p2);
        self.wires.insert(p2, p1);
    }

    pub fn disconnect_ports(&mut self, p1: PortKey, p2: PortKey) {
        self.wires.remove(&p1);
        self.wires.remove(&p2);
    }

    // pub fn get_port(&self, port_key: PortKey) -> &Port {
    //     &self.ports[port_key.0]
    // }

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
}
