use std::{
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
};

#[derive(Debug, Clone)]
pub struct IdxObjVec<T> {
    idxs: Vec<usize>,
    objects: Vec<Option<T>>,
}

impl<T> IdxObjVec<T> {
    pub fn new() -> Self {
        Self {
            idxs: vec![],
            objects: vec![],
        }
    }

    pub fn push(&mut self, obj: T) {
        if self.idxs.is_empty() {
            self.objects.push(Some(obj));
        } else {
            let idx = self.idxs.len() - 1;
            self.objects[self.idxs[idx]] = Some(obj);
            self.idxs.remove(idx);
        }
    }

    pub fn get_object(&self, index: usize) -> Option<&T> {
        if let Some(obj) = &self.objects[index] {
            Some(obj)
        } else {
            None
        }
    }

    pub fn get_mut_object(&mut self, index: usize) -> Option<&mut T> {
        if let Some(obj) = &mut self.objects[index] {
            Some(obj)
        } else {
            None
        }
    }

    pub fn count_objects(&self) -> usize {
        self.objects.len()
    }

    pub fn count_idxs(&self) -> usize {
        self.idxs.len()
    }

    pub fn remove(&mut self, index: usize) {
        self.objects[index] = None;
        self.idxs.push(index);
    }

    pub fn iter_idxs(&self) -> Iter<'_, usize> {
        self.idxs.iter()
    }

    pub fn iter_mut_idxs(&mut self) -> IterMut<'_, usize> {
        self.idxs.iter_mut()
    }

    pub fn iter_objects(&self) -> Iter<'_, Option<T>> {
        self.objects.iter()
    }

    pub fn iter_mut_objects(&mut self) -> IterMut<'_, Option<T>> {
        self.objects.iter_mut()
    }
}

impl<T> Index<usize> for IdxObjVec<T> {
    type Output = Option<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.objects[index]
    }
}

impl<T> IndexMut<usize> for IdxObjVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.objects[index]
    }
}
