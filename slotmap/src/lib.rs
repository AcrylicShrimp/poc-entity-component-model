mod bitmap;
mod id;

pub use bitmap::*;
pub use id::*;

use std::{collections::BTreeMap, num::NonZeroU32};

pub struct SlotMap<T> {
    elements: Vec<T>,
    element_slot_indices: Vec<u32>,
    slot_element_indices: Vec<u32>,
    slot_generations: Vec<NonZeroU32>,
    freed_bitmaps: BTreeMap<u32, Bitmap>,
}

impl<T> SlotMap<T> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            element_slot_indices: Vec::new(),
            slot_element_indices: Vec::new(),
            slot_generations: Vec::new(),
            freed_bitmaps: BTreeMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn get(&self, id: SlotMapId) -> Option<&T> {
        let index = id.index() as usize;

        if self.slot_generations.get(index) != Some(&id.generation()) {
            return None;
        }

        self.elements.get(self.slot_element_indices[index] as usize)
    }

    pub fn get_mut(&mut self, id: SlotMapId) -> Option<&mut T> {
        let index = id.index() as usize;

        if self.slot_generations.get(index) != Some(&id.generation()) {
            return None;
        }

        self.elements
            .get_mut(self.slot_element_indices[index] as usize)
    }

    pub fn add(&mut self, item: T) -> SlotMapId {
        while let Some(mut entry) = self.freed_bitmaps.first_entry() {
            let bitmap = entry.get_mut();

            if bitmap.is_full() {
                entry.remove();
                continue;
            }

            let element_index = self.elements.len();
            self.elements.push(item);

            let freed_local_index = bitmap.find_first_empty_index();
            bitmap.mark_as_filled(freed_local_index);

            let slot_index = bitmap.index() * Bitmap::BITS_PER_WORD + freed_local_index;
            self.element_slot_indices.push(slot_index);
            self.slot_element_indices[slot_index as usize] = element_index as u32;

            let generation = match self.slot_generations[slot_index as usize].checked_add(1) {
                Some(generation) => generation,
                None => NonZeroU32::MIN,
            };
            self.slot_generations[slot_index as usize] = generation;

            return SlotMapId::new(slot_index, generation);
        }

        let element_index = self.elements.len() as u32;
        self.elements.push(item);

        let slot_index = element_index as u32;
        self.element_slot_indices.push(slot_index);
        self.slot_element_indices.push(element_index);

        let generation = NonZeroU32::MIN;
        self.slot_generations.push(NonZeroU32::MIN);

        SlotMapId::new(slot_index, generation)
    }

    pub fn remove(&mut self, id: SlotMapId) -> Option<T> {
        let slot_index = id.index() as usize;
        if self.slot_generations.get(slot_index) != Some(&id.generation()) {
            return None;
        }

        let element_index = self.slot_element_indices[slot_index] as usize;
        if self.elements.len() <= element_index {
            return None;
        }

        let removed = self.elements.swap_remove(element_index);
        self.element_slot_indices.swap_remove(element_index);

        let removed_element_index = self.elements.len();
        if element_index != removed_element_index {
            let dst_slot_index = self.element_slot_indices[element_index] as usize;
            self.slot_element_indices.swap(slot_index, dst_slot_index);
        }

        let removed_bitmap_index = id.index() / Bitmap::BITS_PER_WORD;
        let removed_bitmap_local_index = id.index() % Bitmap::BITS_PER_WORD;
        self.freed_bitmaps
            .entry(removed_bitmap_index)
            .or_insert_with(|| Bitmap::new(removed_bitmap_index))
            .mark_as_empty(removed_bitmap_local_index);

        Some(removed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slotmap_add() {
        let mut map = SlotMap::new();
        let id = map.add(1);
        assert_eq!(map.get(id), Some(&1));
    }

    #[test]
    fn test_slotmap_remove() {
        let mut map = SlotMap::new();
        let id = map.add(1);
        assert_eq!(map.remove(id), Some(1));
        assert_eq!(map.get(id), None);
    }

    #[test]
    fn test_slotmap_complex_1() {
        let mut map = SlotMap::new();
        let id1 = map.add(1);
        let id2 = map.add(2);
        let id3 = map.add(3);

        assert_eq!(map.get(id1), Some(&1));
        assert_eq!(map.get(id2), Some(&2));
        assert_eq!(map.get(id3), Some(&3));

        assert_eq!(map.remove(id1), Some(1));
        assert_eq!(map.remove(id2), Some(2));

        let id4 = map.add(4);
        let id5 = map.add(5);

        assert_eq!(map.get(id4), Some(&4));
        assert_eq!(map.get(id5), Some(&5));

        assert_eq!(map.remove(id4), Some(4));

        let id6 = map.add(6);
        let id7 = map.add(7);
        let id8 = map.add(8);

        assert_eq!(map.get(id6), Some(&6));
        assert_eq!(map.get(id7), Some(&7));
        assert_eq!(map.get(id8), Some(&8));

        assert_eq!(map.get(id1), None);
        assert_eq!(map.get(id2), None);
        assert_eq!(map.get(id3), Some(&3));
        assert_eq!(map.get(id4), None);
        assert_eq!(map.get(id5), Some(&5));
        assert_eq!(map.get(id6), Some(&6));
        assert_eq!(map.get(id7), Some(&7));
        assert_eq!(map.get(id8), Some(&8));

        assert_eq!(map.remove(id7), Some(7));
        assert_eq!(map.remove(id3), Some(3));
        assert_eq!(map.remove(id6), Some(6));
        assert_eq!(map.remove(id8), Some(8));

        assert_eq!(map.get(id1), None);
        assert_eq!(map.get(id2), None);
        assert_eq!(map.get(id3), None);
        assert_eq!(map.get(id4), None);
        assert_eq!(map.get(id5), Some(&5));
        assert_eq!(map.get(id6), None);
        assert_eq!(map.get(id7), None);
        assert_eq!(map.get(id8), None);
    }
}
