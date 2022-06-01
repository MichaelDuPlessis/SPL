pub struct Stack<T> {
    vec: Vec<T>,
}

#[allow(dead_code)]
impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
        }
    }

    pub fn push(&mut self, element: T) {
        self.vec.push(element);
    }

    pub fn append(&mut self, other: &mut T) where T: AsMut<Vec<T>> {
        self.vec.append(other.as_mut());
    }

    pub fn pop(&mut self) -> T {
        self.vec.remove(0)
    }

    pub fn peek(&self) -> &T {
        &self.vec[0]
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn to_vec(self) -> Vec<T> {
        self.vec
    }
}

impl<T> From<Vec<T>> for Stack<T> {
    fn from(vec: Vec<T>) -> Self {
        Self {
            vec
        }
    }
}

impl<T> AsMut<Vec<T>> for Stack<T> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        &mut self.vec
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.vec, f)
    }
}