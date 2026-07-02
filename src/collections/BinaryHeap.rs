
type Ordering<'a, T> = Box<&'a dyn Fn(&T, &T) -> i8>;

//MIN HEAP
pub struct BinaryHeap<'a, T>
{
    array: Vec<T>,
    cmp: Ordering<'a, T>,
    size: usize
}

impl<'a, T> BinaryHeap<'a, T>
{
    pub fn new(cmp: Ordering<'a, T>) -> Self
    {
        BinaryHeap
        {
            array: vec![],
            cmp,
            size: 0,
        }
    }

    pub fn peek(&self) -> &T
    {
        &self.array[0]
    }

    pub fn len(&self) -> usize
    {
        self.size
    }

    pub fn push(&mut self, item: T)
    {
        self.array.push(item);
        self.heapify(self.array.len() - 1);
        self.size += 1;
    }

    pub fn poll(&mut self) -> Option<T>
    {
        if self.len() == 0
        {
            return None
        }

        let last_idx = self.array.len() - 1;

        self.array.swap(0, last_idx);

        let polled_item = self.array.pop().unwrap();

        self.bubble_down(0);

        self.size -= 1;

        Some(polled_item)

    }

    pub fn into_iter(&self) -> std::slice::Iter<T>
    {
        self.array.iter()
    }

    pub fn into_mut_iter(&mut self) -> std::slice::IterMut<T>
    {
        self.array.iter_mut()
    }

    fn heapify(&mut self, idx: usize)
    {
        let parent_idx = idx/2;

        let child_item = &self.array[idx];
        let parent_item  = &self.array[parent_idx];

        let ordering =  (self.cmp)(child_item, parent_item);

        if ordering < 0
        {
            self.array.swap(idx, parent_idx);
            self.heapify(parent_idx);
        }

    }

    fn bubble_down(&mut self, idx: usize)
    {
        let left_child_idx = 2*idx + 1;
        let right_child_idx = 2*(idx+1);

        if left_child_idx >= self.array.len()
        {
            return;
        }
        else if right_child_idx >= self.array.len()
        {
            self.array.swap(left_child_idx, idx);
            // self.bubble_down(left_child_idx);
            return;
        }
        else {            
            let left_child = &self.array[left_child_idx];
            let right_child = &self.array[right_child_idx];
    
            let ordering = (self.cmp)(left_child,right_child);
    
            let swap_idx = if ordering < 0 { left_child_idx } else {right_child_idx};
    
            self.array.swap(swap_idx, idx);
            self.bubble_down(swap_idx);
        }

    }
}