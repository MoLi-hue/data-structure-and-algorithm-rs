#[derive(Default)]
pub struct MaxHeap<T>(Vec<T>);

impl<T> MaxHeap<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.0.len()
    }

    /// # Panics
    /// 索引 `i` 越界可能 panic
    fn parent_index(&self, i: usize) -> Option<usize> {
        if i >= self.size() {
            panic!("index `i` out of bound of `self` size");
        }

        if i == 0 { None } else { Some((i - 1) / 2) }
    }

    /// # Panics
    /// 索引 `i` 越界可能 panic
    fn left_child_index(&self, i: usize) -> Option<usize> {
        if i >= self.size() {
            panic!("index `i` out of bound of `self` size");
        }

        let l = 2 * i + 1;

        if l >= self.size() { None } else { Some(l) }
    }

    /// # Panics
    /// 索引 `i` 越界可能 panic
    fn right_child_index(&self, i: usize) -> Option<usize> {
        if i >= self.size() {
            panic!("index `i` out of bound of `self` size");
        }

        let r = 2 * i + 2;

        if r >= self.size() { None } else { Some(r) }
    }

    fn max_non_leaf_index(&self) -> Option<usize> {
        let len = self.size();
        if len == 0 {
            None
        } else {
            let max_index = len - 1;
            self.parent_index(max_index)
        }
    }
}

impl<T: Ord> MaxHeap<T> {
    pub fn push(&mut self, value: T) {
        let inserted_index = self.size();

        self.0.push(value);

        self.shift_up(inserted_index);
    }

    pub fn pop(&mut self) -> Option<T> {
        let len = self.size();

        if len == 0 {
            None
        } else {
            self.0.swap(0, len - 1);

            let v = self.0.pop();

            if len != 1 {
                self.shift_down(0);
            }

            v
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.0.first()
    }

    // unsafe 应该仅用于内存相关，而非语义逻辑
    // pub unsafe fn peek_mut(&mut self) -> Option<&mut T> {
    //     self.0.first_mut()
    // }

    /// # Panics
    /// 索引 `i` 越界可能 panic
    fn max_index_of_self_and_child(&self, i: usize) -> Option<usize> {
        if i >= self.size() {
            panic!("index `i` out of bound of `self` size");
        }

        let mut max_index = i;

        if let Some(l) = self.left_child_index(i)
            && self.0[l] > self.0[max_index]
        {
            max_index = l;
        }

        if let Some(r) = self.right_child_index(i)
            && self.0[r] > self.0[max_index]
        {
            max_index = r;
        }

        if max_index != i {
            Some(max_index)
        } else {
            None
        }
    }

    /// # Panics
    /// 索引 `i` 越界可能 panic
    fn shift_down(&mut self, mut index: usize) {
        while let Some(max_child) = self.max_index_of_self_and_child(index) {
            self.0.swap(index, max_child);

            index = max_child;
        }
    }

    /// # Panics
    /// 索引 `i` 越界可能 panic
    fn shift_up(&mut self, mut index: usize) {
        while let Some(p) = self.parent_index(index)
            && self.0[index] > self.0[p]
        {
            self.0.swap(index, p);
            index = p;
        }
    }
}

impl<T: Ord> From<Vec<T>> for MaxHeap<T> {
    fn from(value: Vec<T>) -> Self {
        let mut heap = MaxHeap(value);

        if let Some(i) = heap.max_non_leaf_index() {
            for j in (0..=i).rev() {
                heap.shift_down(j);
            }
        }

        heap
    }
}

impl<T: Ord> FromIterator<T> for MaxHeap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from(iter.into_iter().collect::<Vec<_>>())
    }
}
