use core::slice::Iter;

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
pub struct SlotHandle {
    index: usize,
    generation: usize,
}

struct Slot<T> {
    is_allocated: bool,
    generation: usize,
    data: T,
}

pub struct Arena<T> {
    slots: Vec<Slot<T>>,
    count: usize,
}

impl<T> Arena<T> {
    pub fn new() -> Arena<T> {
        Arena {
            slots: Vec::new(),
            count: 0,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn add(&mut self, data: T) -> SlotHandle {
        let mut index = self.slots.len();
        for i in 0..self.slots.len() {
            if !self.slots[i].is_allocated {
                index = i;
                break;
            }
        }
        let mut generation = 0;
        if index >= self.slots.len() {
            let slot = Slot {
                is_allocated: true,
                generation,
                data,
            };
            self.slots.push(slot);
        } else {
            generation = self.slots[index].generation;
        }
        self.count += 1;
        SlotHandle { index, generation }
    }

    pub fn get(&self, handle: SlotHandle) -> Option<&T> {
        if handle.index > self.slots.len() {
            return None;
        }
        match self.slots.get(handle.index) {
            Some(slot) => {
                if slot.generation == handle.generation {
                    Some(&slot.data)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn get_mut(&mut self, handle: SlotHandle) -> Option<&mut T> {
        if handle.index > self.slots.len() {
            return None;
        }
        match self.slots.get_mut(handle.index) {
            Some(slot) => {
                if slot.generation == handle.generation {
                    Some(&mut slot.data)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn remove(&mut self, handle: SlotHandle) {
        let index = handle.index;
        if index > self.slots.len() {
            return;
        }
        self.slots[index].is_allocated = false;
        self.slots[index].generation += 1;
        self.count -= 1;

        if index < self.slots.len() - 1 {
            return;
        }

        let mut i = index;
        while i > 0 && !self.slots.is_empty() && !self.slots[i].is_allocated {
            self.slots.remove(i);
            i -= 1;
        }
    }

    pub fn iter(&self) -> AllocatedSlotIterator<T> {
        AllocatedSlotIterator::new(self.slots.iter())
    }
}

pub struct AllocatedSlotIterator<'a, T> {
    iter: Iter<'a, Slot<T>>,
    index: usize,
}

impl<'a, T> AllocatedSlotIterator<'a, T> {
    fn new(iter: Iter<'a, Slot<T>>) -> AllocatedSlotIterator<'a, T> {
        AllocatedSlotIterator { iter, index: 0 }
    }
}

impl<'a, T> Iterator for AllocatedSlotIterator<'a, T> {
    type Item = SlotHandle;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(slot) => {
                if slot.is_allocated {
                    let handle = SlotHandle {
                        index: self.index,
                        generation: slot.generation,
                    };
                    self.index += 1;
                    Some(handle)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arena_iter() {
        let mut arena: Arena<i32> = Arena::new();
        arena.add(2);
        arena.add(3);
        arena.add(1);
        let iterator = arena.iter();
        let v = vec![2, 3, 1];
        let mut i = 0;
        let mut iterated = false;
        for h in iterator {
            match arena.get(h) {
                Some(x) => {
                    assert_eq!(*x, v[i]);
                }
                None => {
                    panic!("No handle in iterator");
                }
            }
            i += 1;
            iterated = true;
        }
        assert!(iterated);
    }
}
