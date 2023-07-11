struct Hector <T>{
    vector: Vec<T>
}

impl<T> Hector <T>{
    fn new() -> Hector<T>{
        Hector {
            vector: Vec::new(),
        }
    }
    fn with_capacity(capacity: usize) -> Vec<T>{
        let vector = Vec::with_capacity(capacity);
        vector
    }
    fn push(&mut self, value: T){
        self.vector.push(value)
    }
    fn pop(&mut self) -> Option<T>{
        self.vector.pop()
    }
    fn remove(&mut self, index: usize) -> T{
        self.vector.remove(index)
    }
    fn len(&self) -> usize{
        self.vector.len()
    }
    fn get(&self, reference: usize) -> Option<&T>{
        self.vector.get(reference)
    }
    fn resize(&mut self, newsize: usize, value: T) where T: Clone,
    {
        self.vector.resize(newsize, value)
    }
}

fn main() {
}
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn push() {
        let mut vec = Hector::new();
        vec.push(1);
        vec.push(2);
        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn remove() {
        let mut vec = Hector::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.remove(1), 2);
        assert_eq!(vec.len(), 2);
    }
    #[test]
    fn get(){
        let mut vec = Hector::new();
        vec.push(2);
        vec.push(4);
        vec.push(6);
        assert_eq!(vec.get(1), Some(&4))
    }
    #[test]
    fn resize(){
        let mut vec = Hector::new();
        vec.push(2);
        vec.push(4);
        vec.push(6);
        vec.resize(5, 3);
        assert_eq!(vec.get(3), Some(&3));
        assert_eq!(vec.get(4), Some(&3));
    }
}
