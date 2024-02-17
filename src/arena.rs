pub type ArenaRef = usize;
pub enum ArenaError {
    IndexOutOfBounds
}

pub struct Arena<T> {
    active_pool: Vec<Option<T>>,
    inactive_pool: Vec<ArenaRef>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self {
            active_pool: Vec::new(),
            inactive_pool: Vec::new(),
        }
    }

    pub fn get(&self, index: ArenaRef) -> Option<&T> {
        match self.active_pool.get(index) {
            Some(elem) => elem.as_ref(),
            None => None
        }
    }

    pub fn get_mut(&mut self, index: ArenaRef) -> Option<&mut T> {
        match self.active_pool.get_mut(index) {
            Some(elem) => elem.as_mut(),
            None => None
        }
    }

    pub fn add(&mut self, elem: T) -> ArenaRef {
        if let Some(idx) = self.inactive_pool.pop() {
            self.active_pool[idx] = Some(elem);
            idx
        } else {
            self.active_pool.push(Some(elem));
            self.active_pool.len() - 1
        }
    }

    pub fn remove(&mut self, index: ArenaRef) -> Result<(), ArenaError> {
        if index >= self.active_pool.len() {
            Err(ArenaError::IndexOutOfBounds)
        } else {
            self.active_pool[index] = None;
            self.inactive_pool.push(index);
            Ok(())
        }
    }
}
