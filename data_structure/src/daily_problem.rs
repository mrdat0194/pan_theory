use std::{
    cell::RefCell,
    collections::HashSet,
    iter::ExactSizeIterator,
    mem,
};

// https://codereview.stackexchange.com/questions/251255/how-rusty-is-my-generic-union-find-implementation
pub fn main() {

    #[derive(Clone)]
    pub struct DisjointSet<T: Eq> {
        roots: HashSet<usize>,
        nodes: Vec<RefCell<Node<T>>>,
    }

    #[derive(Default, Clone)]
    pub struct Node<T> {
        elem: T,
        parent_idx: usize,
        rank: usize,
        // We use this to be able to iterate
        // on each of our subsets.
        next: usize,
    }

    // impl fmt::Display for Circle {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         write!(f, "Circle of radius {}", self.radius)
    //     }
    // }

    impl<T: Eq + std::fmt::Debug > DisjointSet<T> {
        /// Creates an empty `DisjointSet`.
        pub fn new() -> Self {
            Self {
                nodes: vec![],
                roots: HashSet::new(),
            }
        }

        /// Creates a new `DisjointSet` with the given capacity.
        pub fn with_capacity(capacity: usize) -> Self {
            Self {
                nodes: Vec::with_capacity(capacity),
                roots: HashSet::new(),
            }
        }

        /// Returns the number of subsets.
        pub fn num_sets(&self) -> usize {
            self.roots.len()
        }

        /// Returns the number of total elements in all subsets.
        pub fn num_elements(&self) -> usize {
            self.nodes.len()
        }

        /// Returns true if the given element is present in the `DisjointSet`.
        pub fn contains(&self, elem: &T) -> bool {
            self.position(elem).is_some()
        }

        /// Returns the index of the given element if it exists, or None otherwise.
        pub fn position(&self, elem: &T) -> Option<usize> {
            self.nodes.iter().position(|e| &e.borrow().elem == elem)
        }

        /// Adds a new set with a single, given element to
        /// the `DisjointSet`. Returns an Err with the elem
        /// if it was already present in any set, otherwise
        /// returns a Ok(usize) with the index of the element.
        pub fn make_set(&mut self, elem: T) -> Result<usize, T> {
            if !self.contains(&elem) {
                // This is the index where the node will be inserted,
                // thanks to the magic of zero-indexing.
                let insertion_idx = self.nodes.len();

                self.nodes.push(RefCell::new(Node {
                    elem,
                    parent_idx: insertion_idx,
                    rank: 0,
                    next: insertion_idx,
                }));

                self.roots.insert(insertion_idx);

                Ok(insertion_idx)
            } else {
                Err(elem)
            }
        }

        /// If present, returns an immutable reference to the element at `elem_idx`.
        pub fn get(&self, elem_idx: usize) -> Option<&T> {
            // Nothing in our code actually mutates node.elem: T using &self.
            // Even find_root_idx uses interior mutability only
            // to modify node.parent. And the caller can't
            // call get_mut or iter_mut_set while the &T here is
            // still in scope. So it all works out!
            Some(unsafe { &*self.get_raw(elem_idx)? })
        }

        /// If present, returns a mutable reference to the element at `elem_idx`.
        pub fn get_mut(&mut self, elem_idx: usize) -> Option<&mut T> {
            // RefCall::get_mut is used rarely, but here it's appropriate:
            // As long as the &mut T from this is still in scope,
            // the caller won't be able to use any other methods,
            // so interior mutability isn't a concern.
            Some(&mut self.nodes.get_mut(elem_idx)?.get_mut().elem)
        }

        /// If present, returns a raw pointer to the element at `elem_idx`.
        fn get_raw(&self, elem_idx: usize) -> Option<*mut T> {
            unsafe { Some(&mut (*self.nodes.get(elem_idx)?.as_ptr()).elem as *mut _) }
        }

        // /// Returns an `&T` iterator over all elements in the set
        // /// elem_idx belongs to, if it exists.
        // // We use both applicable Iterator types here to give the caller
        // // the maximum possible flexbility when using the returned value.
        // pub fn iter_set(
        //     &self,
        //     elem_idx: usize,
        // ) -> Option<impl ExactSizeIterator<Item = &T> + DoubleEndedIterator> {
        //     Some(
        //         self.get_set_idxs(elem_idx)?
        //             .into_iter()
        //             .map(move |i| self.get(i).unwrap()),
        //     )
        // }
        //
        // /// Returns an `&mut T` iterator over all elements in the set
        // /// elem_idx belongs to, if it exists.
        // pub fn iter_mut_set(
        //     &mut self,
        //     elem_idx: usize,
        // ) -> Option<impl ExactSizeIterator<Item = &mut T> + DoubleEndedIterator> {
        //     let set_idxs = self.get_set_idxs(elem_idx)?;
        //
        //     Some(set_idxs.into_iter().map(move |i| {
        //         // In reality this is safe because there'll
        //         // be no duplicate indexes. But Rust doesn't
        //         // have any way of knowing that.
        //         unsafe { &mut *(self.get_mut(i).unwrap() as *mut _) }
        //     }))
        // }
        //
        // pub fn iter_all_sets(
        //     &self,
        // ) -> impl ExactSizeIterator<Item = impl ExactSizeIterator<Item = &T> + DoubleEndedIterator>
        // + DoubleEndedIterator {
        //     // Put roots into a Vec to satisfy DoubleEndedIterator
        //     let roots = self.roots.iter().collect::<Vec<_>>();
        //
        //     roots.into_iter().map(move |&r| self.iter_set(r).unwrap())
        // }
        //
        // pub fn iter_mut_all_sets(
        //     &mut self,
        // ) -> impl ExactSizeIterator<Item = impl ExactSizeIterator<Item = &mut T> + DoubleEndedIterator>
        // + DoubleEndedIterator {
        //     // This function can't be as simple as iter_all_sets,
        //     // because Rust won't like it if we just straight up take
        //     // &mut self several times over.
        //     self.roots
        //         .iter()
        //         .map(|&root| {
        //             self.get_set_idxs(root)
        //                 .unwrap()
        //                 .into_iter()
        //                 .map(|i| {
        //                     // No duplicate indexes means that using this
        //                     // pointer as a &mut T is safe. We can't
        //                     // use get_mut here because that takes &mut self.
        //                     unsafe { &mut *self.get_raw(i).unwrap() }
        //                 })
        //                 .collect::<Vec<_>>()
        //         })
        //         // In order to avoid the closures that borrow
        //         // self outliving the function itself, we collect
        //         // their results and then turn them back into iterators.
        //         .collect::<Vec<_>>()
        //         .into_iter()
        //         .map(|v| v.into_iter())
        // }

        /// Returns Some(true) if the elements at both the given indexes
        /// are in the same set, or None of either of them aren't present altogether.
        pub fn same_set(&self, elem1_idx: usize, elem2_idx: usize) -> Option<bool> {
            // The ? ensures this'll short-circuit and return None if either of the indexes are None,
            // meaning we don't end up returning Some(true) if both elements don't exist.
            Some(self.find_root_idx(elem1_idx)? == self.find_root_idx(elem2_idx)?)
        }

        pub fn union(&mut self, elem_x_idx: usize, elem_y_idx: usize) -> Option<bool> {
            let (mut x_root_idx, mut y_root_idx) = (
                self.find_root_idx(elem_x_idx)?,
                self.find_root_idx(elem_y_idx)?,
            );

            // We don't have to do anything if this is the case.
            // Also, if we didn't check this, we'd panic below because
            // we'd attempt two mutable borrowings of the same RefCell.
            if x_root_idx == y_root_idx {
                return Some(false);
            }

            let (mut x_root, mut y_root) = (
                self.nodes[x_root_idx].borrow_mut(),
                self.nodes[y_root_idx].borrow_mut(),
            );

            if x_root.rank < y_root.rank {
                // Must use mem::swap here. If we shadowed,
                // it'd go out of scope when the if block ended.
                mem::swap(&mut x_root_idx, &mut y_root_idx);
                mem::swap(&mut x_root, &mut y_root);
            }

            // Now x_root.rank >= y_root.rank no matter what.
            // Therefore, make X the parent of Y.
            y_root.parent_idx = x_root_idx;
            self.roots.remove(&y_root_idx);
            if x_root.rank == y_root.rank {
                x_root.rank += 1;
            }

            // Drop the RefMuts so we can check self.last,
            // which needs to immutably borrow, without conflicts.
            drop(x_root);
            drop(y_root);

            let x_last_idx = self.last(x_root_idx).unwrap();
            let mut x_last = self.nodes[x_last_idx].borrow_mut();
            x_last.next = y_root_idx;

            Some(true)
        }

        /// Returns the index of the root of the subset
        /// `elem_idx` belongs to, if it exists.
        pub fn find_root_idx(&self, elem_idx: usize) -> Option<usize> {
            if self.roots.contains(&elem_idx) {
                return Some(elem_idx);
            }

            let mut curr_idx = elem_idx;
            let mut curr = self.nodes.get(curr_idx)?.borrow_mut();

            while curr.parent_idx != curr_idx {
                let parent_idx = curr.parent_idx;
                let parent = self.nodes[parent_idx].borrow_mut();

                // Set the current node's parent to its grandparent.
                // This is called *path splitting*: (see the Wikipedia
                // page for details) a simpler to implement, one-pass
                // version of path compression that also, apparently,
                // turns out to be more efficient in practice.
                curr.parent_idx = parent.parent_idx;

                // Move up a level for the next iteration
                curr_idx = parent_idx;
                curr = parent;
            }

            Some(curr_idx)
        }

        /// Returns the last element of the subset with
        /// `elem_idx` in it, if it exists.
        fn last(&self, elem_idx: usize) -> Option<usize> {
            self.get_set_idxs(elem_idx)?.pop()
        }

        /// Returns the indexes of all the items in the subset
        /// `elem_idx` belongs to in arbitrary order, if it exists.
        fn get_set_idxs(&self, elem_idx: usize) -> Option<Vec<usize>> {
            let mut curr_idx = self.find_root_idx(elem_idx)?;
            let mut curr = self.nodes[curr_idx].borrow();

            let mut set_idxs = Vec::with_capacity(self.num_elements());

            // We can't check the condition up here
            // using while because that would make
            // it so the last node is never pushed.
            loop {
                set_idxs.push(curr_idx);

                // This is the last node
                if curr_idx == curr.next {
                    break;
                }

                curr_idx = curr.next;
                curr = self.nodes[curr.next].borrow();
            }

            set_idxs.shrink_to_fit();

            Some(set_idxs)
        }

        fn add_synonyms(&mut self, w1: T, w2: T) {
            let find_set_w1 = self.nodes.iter().position(|x| x.borrow().elem == w1);
            let find_set_w2 = self.nodes.iter().position(|x| x.borrow().elem == w2);
            // println!("{:?} ", find_set_w1);
            if find_set_w1 == None && find_set_w2 == None {
                let a = self.make_set(w1).unwrap_or_default();
                let b = self.make_set(w2).unwrap_or_default();

                self.union(a, b);
            } else {
                if find_set_w1 != None {
                    let t = self.find_root_idx(find_set_w1.unwrap_or_default());

                    let t1 = self.nodes[t.unwrap_or_default()].borrow_mut().parent_idx;
                    // println!("{:?} ", &w1);

                    let b = self.make_set(w2).unwrap_or_default();

                    self.union(t1, b);


                } else {
                    let t = self.find_root_idx(find_set_w2.unwrap_or_default());

                    let t1 = self.nodes[t.unwrap_or_default()].borrow_mut().parent_idx;

                    // println!("{:?} ", &w2);

                    let b = self.make_set(w1).unwrap_or_default();

                    self.union(t1, b);
                }
            }
        }

        fn are_synonymous(&self, w1: T , w2: T) -> bool {
            let find_set_w1 = self.nodes.iter().position(|x| x.borrow().elem == w1);
            let find_set_w2 = self.nodes.iter().position(|x| x.borrow().elem == w2);
            if (find_set_w1 == None || find_set_w2 == None) {
                return false
            } else {
                return self.find_root_idx(find_set_w1.unwrap()) == self.find_root_idx(find_set_w2.unwrap());
            }
        }
    }

    use std::fmt;

    fn synonym_queries<'a>(synonym_words : Vec<(&'static str, &'static str)>, queries: Vec<(&'static str, &'static str)>) -> DisjointSet<&'a str> {
        let mut ds = DisjointSet::new();

        let  v1_iter = synonym_words.into_iter();

        for (w1,w2) in v1_iter {
            // println!("{:?} ", w1);
            ds.add_synonyms(w1, w2);
        }

        // println!("{:?} ", ds.are_synonymous("large","huge"));

        let mut output = Vec::new();;
        let  v2_iter = queries.into_iter();

        for (q1, q2) in v2_iter {
            // println!("{:?} ", q1);
            // let (q1, q2) = (q1.split(' ').collect(), q2.split(' ').collect());
            let q3: Vec<&str> = q1.split(' ').collect();
            let q4: Vec<&str> = q2.split(' ').collect();
            // println!("{:?} ", q3.len());
            if q3.len() != q4.len() {
                output.push(false);
                println!("{:?} ", output);
                break;

            }

            // println!("{:?} ", q1);

            let mut result = true;

            for i in (0..q3.len()) {
                let (w1, w2) = (q3[i], q4[i]);

                // println!("{:?} ", w2);
                if w1 == w2 {
                    continue;
                } else {
                    if ds.are_synonymous(&w1, &w2) {
                        continue;
                    } else {
                        // println!("{:?} ", ds.are_synonymous(&w1, &w2));
                        result = false;
                        break;
                    }
                }
            }
            output.push(result);
            println!("{:?} ", output);
            //     return output;
        }

        return ds
    };


    let dictionary = vec![("big", "large"), ("eat", "consume"),("big", "huge"),("wants", "loves")];
    let queries = vec![("He wants to eat big food.", "He loves to consume huge food.")];

    let  x = synonym_queries(dictionary, queries);



    println!("{:?} ", x.roots);
    // let kk = x.nodes.iter().position(|x| x.borrow().elem == "huge");
    // println!("{:?} ", &kk);
    // println!("{:?} ", x.find_root_idx(kk.unwrap()));
    // println!("{:?} ", x.nodes[1].borrow_mut().parent_idx);

}