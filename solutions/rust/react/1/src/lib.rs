use std::collections::{BTreeMap, HashSet};

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct InputCellId(u32);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComputeCellId(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CallbackId(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct InputCell<T> {
    value: T,
}

type ComputeFunction<T> = Box<dyn Fn(&[T]) -> T>;

struct ComputeCell<T> {
    dependencies: Vec<CellId>,
    compute_func: ComputeFunction<T>,
    callback_ids: HashSet<CallbackId>,
}

pub struct Reactor<'a, T> {
    input_cells: BTreeMap<InputCellId, InputCell<T>>,
    compute_cells: BTreeMap<ComputeCellId, ComputeCell<T>>,
    callbacks: BTreeMap<CallbackId, Box<dyn FnMut(T) + 'a>>,
    compute_value_cache: BTreeMap<ComputeCellId, T>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            input_cells: BTreeMap::new(),
            compute_cells: BTreeMap::new(),
            callbacks: BTreeMap::new(),
            compute_value_cache: BTreeMap::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let id = if let Some(InputCellId(max_id)) = self.input_cells.keys().last() {
            max_id + 1
        } else {
            0
        };

        self.input_cells
            .insert(InputCellId(id), InputCell { value: initial });

        InputCellId(id)
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        let id = if let Some(ComputeCellId(max_id)) = self.compute_cells.keys().last() {
            max_id + 1
        } else {
            0
        };

        for dep in dependencies {
            match dep {
                CellId::Input(input_cell_id) => {
                    if !self.input_cells.contains_key(input_cell_id) {
                        return Err(*dep);
                    }
                }
                CellId::Compute(compute_cell_id) => {
                    if !self.compute_cells.contains_key(compute_cell_id) {
                        return Err(*dep);
                    }
                }
            }
        }

        let compute_cell_id = ComputeCellId(id);

        self.compute_cells.insert(
            compute_cell_id,
            ComputeCell {
                dependencies: dependencies.to_vec(),
                compute_func: Box::new(compute_func),
                callback_ids: HashSet::new(),
            },
        );

        self.compute_value_cache.insert(
            compute_cell_id,
            self.value(CellId::Compute(compute_cell_id))
                .expect("Newly created compute_cell should compute."),
        );

        Ok(compute_cell_id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(input_cell_id) => {
                self.input_cells.get(&input_cell_id).map(|cell| cell.value)
            }
            CellId::Compute(compute_cell_id) => {
                if let Some(compute_cell) = self.compute_cells.get(&compute_cell_id) {
                    let dependency_values = compute_cell
                        .dependencies
                        .iter()
                        .map(|cell_id| self.value(*cell_id).unwrap())
                        .collect::<Vec<_>>();

                    Some((compute_cell.compute_func)(dependency_values.as_slice()))
                } else {
                    None
                }
            }
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        if self.input_cells.contains_key(&id) {
            self.input_cells
                .entry(id)
                .and_modify(|e| e.value = new_value);

            // find all compute cells that have changed
            let changed = self
                .compute_cells
                .iter()
                .filter_map(|(cell_id, cell)| {
                    if *self.compute_value_cache.get(cell_id).unwrap()
                        != self.value(CellId::Compute(*cell_id)).unwrap()
                    {
                        Some((cell_id, cell))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            for (compute_cell_id, compute_cell) in changed {
                let compute_cell_value = self
                    .value(CellId::Compute(*compute_cell_id))
                    .expect("compute_cell should compute");

                for callback_id in &compute_cell.callback_ids {
                    (self.callbacks.get_mut(callback_id).unwrap())(compute_cell_value);
                }

                let new_compute_cell_value = self
                    .value(CellId::Compute(*compute_cell_id))
                    .expect("compute_cell should compute");

                self.compute_value_cache
                    .insert(*compute_cell_id, new_compute_cell_value);
            }

            true
        } else {
            false
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        if let Some(cell) = self.compute_cells.get_mut(&id) {
            let id = if let Some(CallbackId(max_id)) = self.callbacks.keys().last() {
                max_id + 1
            } else {
                0
            };

            let callback_id = CallbackId(id);

            self.callbacks.insert(callback_id, Box::new(callback));

            cell.callback_ids.insert(callback_id);

            Some(callback_id)
        } else {
            None
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        if let Some(compute_cell) = self.compute_cells.get_mut(&cell) {
            if self.callbacks.contains_key(&callback) {
                compute_cell.callback_ids.remove(&callback);

                if self
                    .compute_cells
                    .iter()
                    .all(|(_, cell)| !cell.callback_ids.contains(&callback))
                {
                    self.callbacks.remove(&callback);
                }

                Ok(())
            } else {
                Err(RemoveCallbackError::NonexistentCallback)
            }
        } else {
            Err(RemoveCallbackError::NonexistentCell)
        }
    }
}
