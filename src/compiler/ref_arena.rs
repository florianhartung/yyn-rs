use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Clone, Default, Debug)]
pub struct Arena<T> {
    inner_arena: Rc<RefCell<InnerArena<T>>>,
}

#[derive(Default, Debug)]
struct InnerArena<T>(generational_arena::Arena<T>);

#[derive(Debug)]
pub struct ArenaRef<T> {
    idx: generational_arena::Index,
    arena: Rc<RefCell<InnerArena<T>>>,
}

impl<T> Clone for ArenaRef<T> {
    fn clone(&self) -> Self {
        Self {
            idx: self.idx,
            arena: self.arena.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.idx = source.idx;
        self.arena = source.arena.clone()
    }
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self {
            inner_arena: Rc::new(RefCell::new(InnerArena(generational_arena::Arena::new()))),
        }
    }
    pub fn insert(&self, t: T) -> ArenaRef<T> {
        let idx = self.inner_arena.borrow_mut().0.insert(t);
        ArenaRef {
            idx,
            arena: self.inner_arena.clone(),
        }
    }
}

impl<T> ArenaRef<T> {
    pub fn get(&self) -> impl Deref<Target = T> + '_ {
        Ref::map(self.arena.borrow(), |InnerArena(arena)| {
            arena.get(self.idx).expect("arena ref to be valid because this arena does not allow its elements to be deleted")
        })
    }

    pub fn get_mut(&self) -> impl DerefMut<Target = T> + '_ {
        RefMut::map(self.arena.borrow_mut(), |InnerArena(arena)| {
            arena.get_mut(self.idx).expect("arena ref to be valid because this arena does not allow its elements to be deleted")
        })
    }
}
