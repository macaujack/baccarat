#[derive(Debug, Clone)]
pub struct UndoList<T: Clone, const SIZE: usize> {
    foremost_index: usize,
    current_index: usize,
    length: usize,
    data: Vec<T>,
}

impl<T: Clone, const SIZE: usize> UndoList<T, SIZE> {
    pub fn new() -> Self {
        Self {
            foremost_index: 0,
            current_index: 0,
            length: 0,
            data: Vec::new(),
        }
    }

    pub fn append(&mut self, new_item: T) {
        if self.length == 0 {
            self.data.resize(SIZE, new_item);
            self.length = 1;
            return;
        }
        self.length = self.length - (self.foremost_index + SIZE - self.current_index) % SIZE + 1;
        self.current_index = (self.current_index + 1) % SIZE;
        self.data[self.current_index] = new_item.clone();
        self.foremost_index = self.current_index;
        if self.length > SIZE {
            self.length = SIZE;
        }
    }

    pub fn undo(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        let backmost_index = (self.foremost_index + SIZE + 1 - self.length) % SIZE;
        if self.current_index == backmost_index {
            return None;
        }
        self.current_index = (self.current_index + SIZE - 1) % SIZE;
        Some(self.data[self.current_index].clone())
    }

    pub fn redo(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        if self.current_index == self.foremost_index {
            return None;
        }
        self.current_index = (self.current_index + 1) % SIZE;
        Some(self.data[self.current_index].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undo_list() {
        let mut undo_list = UndoList::<i32, 5>::new();
        assert_eq!(undo_list.length, 0);
        undo_list.append(40);
        assert_eq!(undo_list.length, 1);
        assert_eq!(undo_list.undo(), None);
        assert_eq!(undo_list.redo(), None);

        undo_list.append(41);
        undo_list.append(42);

        assert_eq!(undo_list.undo().unwrap(), 41);
        assert_eq!(undo_list.length, 3);

        assert_eq!(undo_list.redo().unwrap(), 42);
        assert_eq!(undo_list.undo().unwrap(), 41);
        assert_eq!(undo_list.undo().unwrap(), 40);
        assert_eq!(undo_list.undo(), None);
        assert_eq!(undo_list.redo().unwrap(), 41);
        assert_eq!(undo_list.undo().unwrap(), 40);

        undo_list.append(51);
        undo_list.append(52);
        undo_list.append(53);
        undo_list.append(54);
        assert_eq!(undo_list.length, 5);
        undo_list.append(55);
        assert_eq!(undo_list.length, 5);

        assert_eq!(undo_list.undo().unwrap(), 54);
        assert_eq!(undo_list.undo().unwrap(), 53);
        undo_list.append(64);
        assert_eq!(undo_list.length, 4);
        assert_eq!(undo_list.redo(), None);
        undo_list.append(65);
        assert_eq!(undo_list.length, 5);
        undo_list.append(66);
        assert_eq!(undo_list.length, 5);

        assert_eq!(undo_list.undo().unwrap(), 65);
        assert_eq!(undo_list.undo().unwrap(), 64);
        assert_eq!(undo_list.undo().unwrap(), 53);
        assert_eq!(undo_list.undo().unwrap(), 52);
        assert_eq!(undo_list.undo(), None);
        assert_eq!(undo_list.redo().unwrap(), 53);

        undo_list.append(74);
        assert_eq!(undo_list.length, 3);
    }
}
