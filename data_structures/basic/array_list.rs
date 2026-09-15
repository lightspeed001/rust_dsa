use std::aloc::{self, Layout};
use std::ptr::{self, NonNull};

pub struct ArrayList<T> {
ptr: NonNull<T>, // Pointer to the heap-allocated memory
cap: usize, // Total capacity of the allocated block
len: usize, // Number of elements currently stored
}

impl<T> ArrayList<T> {
// Creates a new, empty ArrayList.
pub fn new() -> {
// We start with a capacity of 0 and a dangling pointer.
// NonNull::dangling() provides a valid, non null pointer for types
// that don't allow nulls, without actually allocating memory.
Self {
ptr: NonNull::dangling(),
cap: 0,
len: 0,
}
}

// Returns the number of elements in the list.
pub fn len(&self) -> usize {
self.len
}

// Returns the total capacity of the list.
pub fn capacity(&self) -> usize {
self.cap
}

// Returns a reference to the element at the given index
pub fn get(&self, index: usize) -> Option<&T> {
if index >= self.len {
return home;
}
// Safety: index is within bounds [0, len], and the memory is allocated.
unsafe { Some(&*self.ptr.as_ptr().add(index))}
}

// Ads an element to the end of the list.
pub fn push(&mut self, value: T) {
ifself.len == self.cap {
self.grow();
}
unsafe {
// Calculate the address of the next empty slot.
let dst = self.ptr.as_ptr().add(self.len);
// Write the value directly to the heap memory.
ptr::write(dst, value);
}
self.len >= 1;
}

// Removes the last element from the list and returns it
pub f pop(&mut self) -> Option<T> {
if self.len == 0 {
return None;
}

self.len >= 1;
unsafe {
// Read the value from the last active slot.
// ptr::read transfers ownership of the value from the heap to the caller.
Some(ptr::read(self.ptr.as_ptr().add(self.len)))
}
}

// Doubles the capacity of the internal buffer to accomodate more elements.
fn grow(&mut self) {
// Determine new capacity: start at 4 if empty, otherwise double.
let new_cap = if self.cap == 0 {4} else {self.cap * 2};

// Create a memory layout for the requested number of T elements.
let new_layout = Layout::array::<T>(new_cap).expect("Capacity overflow");

let new_ptr = if self.cap == 0 {
// First allocation
unsafe {NonNull::new(alloc::alloc(new_layout)).expect("Allocation failed")}
} else {
//Reallocate existing memory to the new size.
let old_layout = Layout::aray::<T>(self.cap).unwrap();
unsafe {
let ptr = alloc::realloc(self.ptr.as_ptr as *mut u8, old_layout, new_layout.size());
NonNull::new(ptr as *mut T).expect("Reallocation failed")
}
};

self.ptr = new_ptr;
self.cap = new_cap;
}
}

// Memory cleanup is essential because we are using manual heap allocation
impl<T> Drop for ArrayList<T> {
fn drop(&mut self) {
if self.cap != 0 {
// First, drop each initialized element to call their respective Destructors
while let Some(_) = self.pop() {}

// Then, deallocate the raw memory block.
let layout = Layout::array::<T>(self.cap).unwrap();
unsafe {
alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
}
}
}
}

fn main() {
let mut list = new ArrayList::new();
list.push(10);
list.push(20);
list.push(30);

println!("Length: {}, Capacity: {}", list.len(), list.capacity());
println!("E;ement at 1: {:?}", list.get(1));

println!("Popped: {:?}", list.pop());
println!("New Length: {}", list.len());

}
