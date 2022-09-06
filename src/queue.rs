use std::{net::TcpStream};

// Черга з'єднань
#[derive(Debug)]
pub struct Queue {
  max: usize,                         // Максимальна ємкість
  pub len: usize,                     // Поточна довжина
  first: usize,                       // Перший елемент черги
  last: usize,                        // Останій елемент черги
  data: Vec<Option<TcpStream>>,       // Дані
}

impl Queue {
  pub fn new(max: usize) -> Queue {
    let mut list: Vec<Option<TcpStream>> = Vec::with_capacity(max);
    for _ in 0..max {
      list.push(None);
    }
    Queue {
      max,
      len: 0,
      first: 0, 
      last: max - 1,
      data: list,
    }
  }

  pub fn push(&mut self, tcp: TcpStream) -> Option<TcpStream> {
    if self.len == self.max {
      return Some(tcp);
    }
    self.len += 1;
    let mut next = self.last + 1;
    if next == self.max {
      next = 0;
    }
    self.last = next;
    self.data[next].replace(tcp);
    None
  }

  pub fn take(&mut self) -> Option<TcpStream> {
    if self.len == 0 {
      return None;
    }
    self.len -= 1;
    let v = self.data[self.first].take();
    let mut next = self.first + 1;
    if next == self.max {
      next = 0;
    }
    self.first = next;
    v
  }
}