use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct Stack<T, const CAPACITY: usize> {
  pub collection: [T; CAPACITY],
  pub iter_index: usize
}

impl<T, const CAPACITY: usize> Default for Stack<T, CAPACITY>
where
    T: Default + Copy,
{
    fn default() -> Self {
      Self { collection: [T::default(); CAPACITY], iter_index: usize::default() }
    }
}

impl<T, const CAPACITY: usize> Stack<T, CAPACITY> {
  pub fn push(&mut self, item: T) {
    let new_position = (self.iter_index + CAPACITY) % CAPACITY;

    self.collection[new_position] = item;
    self.iter_index = (self.iter_index + 1) % CAPACITY;
  }
}

#[cfg(test)]
mod test {
    use super::Stack;

  #[test]
  fn push_new_item() {
      let mut stack: Stack<f64, 3> = Stack::default();
      stack.push(1.0);

      // Confirm that item is collected and iteration index incremented
      assert_eq!(stack.collection[0], 1.0);
      assert_eq!(stack.iter_index, 1);
  }

  #[test]
  fn back_to_zero_index() {
      use std::iter::FromIterator;

      const MAX_SIZE: usize = 3;
      let values: Vec<i32> = Vec::from_iter(0..(MAX_SIZE as i32 + 1));
      let mut stack: Stack<f64, MAX_SIZE> = Stack::default();

      for val in values.iter() {
        stack.push(*val as f64);
      }

      // Confirm there is no overflow and value at max size + 1 is at position 0
      assert_eq!(stack.collection[0], *values.last().unwrap() as f64);
      assert_eq!(stack.iter_index, 1);
  }
}
