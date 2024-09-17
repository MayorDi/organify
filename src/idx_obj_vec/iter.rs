use super::IdxObjVec;

pub struct Iter<'a, T> {
    idx_obj_vec: &'a IdxObjVec<T>,
    index: usize,
}

impl<'a, T> Iter<'a, T> {
    pub fn new(idx_obj_vec: &'a IdxObjVec<T>) -> Self {
        Self {
            idx_obj_vec,
            index: 0,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.idx_obj_vec.objects.len() {
            return None;
        }

        if let Some(obj) = &self.idx_obj_vec[self.index] {
            let res = Some(obj);
            self.index += 1;

            return res;
        }

        self.index += 1;
        let res = self.next();

        res
    }
}
