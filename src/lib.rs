pub struct ArrayQueue<T> {
    queue : Vec<Option<T>>,
    j : usize,
    n : usize
}

impl<T> ArrayQueue<T> 
where T: Clone {
    pub fn new() -> ArrayQueue<T> {
        let mut aq = ArrayQueue {
            queue : Vec::with_capacity(1),
            j : 0,
            n : 0
        };
        aq.queue.push(None);
        aq
    }

    pub fn add(&mut self, value : T) {
        let capacity = self.queue.capacity();
        if self.n + 1 > capacity {
            // reserves additional specified amounts
            self.queue.resize((self.n+1) * 2, None);
        }
        let idx : usize;
        idx = (self.j + self.n) % self.queue.capacity();
        println!("{} {} adding to {}", self.j, self.n, idx);
        //self.queue.insert(idx, Some(value));
        self.queue[idx] = Some(value);
        self.n += 1;
    }

    pub fn remove(&mut self) -> Option<T> {
        let value = self.queue[self.j].clone();

        if value.is_none() {
            return value;
        }
        self.queue[self.j] = None;

        self.j += 1;
        self.j %= self.queue.capacity();

        if self.n - 1 < self.queue.capacity() / 2 {
            self.queue.shrink_to_fit();
        }
        self.n -= 1;
        value
    }
}

#[cfg(test)]
mod tests {
    use crate::ArrayQueue;

    #[test]
    fn test_array_queue_add() {
        let mut arr_queue = ArrayQueue::<i32>::new();

        fn print_q(arr_q : &ArrayQueue::<i32>) {
            println!("{:?}", arr_q.queue);
        }

        arr_queue.add(1);
        
        assert_eq!(arr_queue.queue.capacity(), 1);

        arr_queue.add(2);
        print_q(&arr_queue);
        assert_eq!(arr_queue.queue.capacity(), 4);

        arr_queue.add(3);
        print_q(&arr_queue);
        assert_eq!(arr_queue.queue.capacity(), 4);

        let popped_value = arr_queue.remove();
        assert_eq!(popped_value, Some(1));
        assert_eq!(arr_queue.queue.capacity(), 4);
        print_q(&arr_queue);

        arr_queue.add(4);
        print_q(&arr_queue);
        assert_eq!(arr_queue.queue.capacity(), 4);

        arr_queue.add(5);
        print_q(&arr_queue);
        assert_eq!(arr_queue.queue.capacity(), 4);

        let popped_value = arr_queue.remove();
        assert_eq!(popped_value, Some(2));
        print_q(&arr_queue);
    }
}
