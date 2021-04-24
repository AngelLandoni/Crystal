use std::iter;

/// A list of vectors which allow access any index (except negatives).
/// 
/// # Use cases.
///     * If the index is bigger than the current size.
///         * and it is a read it will return None.
///         * and it is write it will expand the buffer to the needed size (always in N sizes). 
pub struct BlockVec<T, const N: usize> {
    // The current number of items in the Vec.
    number_of_items: usize,

    /// A queue of pointers to blocks.
    blocks: Vec<Vec<Option<T>>>
}

/// Provides default constructors for `BlockVec`.
impl<T, const N: usize> BlockVec<T, N> {
    /// Creates and returns a new `BlockVec`.
    pub fn new() -> Self {
        let mut new_self = Self {
            number_of_items: 0,
            blocks: Vec::new()
        };

        // Init the vector with only 1 block.
        new_self.append_empty_blocks(1);

        new_self
    }
}

/// Provides item handling.
impl<T, const N: usize> BlockVec<T, N> {
    /// Pushes a new item into the data structure.
    ///
    /// # Arguments
    ///
    /// `item` - The item to be inserted.
    /// `index` - The position of the element. 
    pub fn set(&mut self, item: T, index: usize) {
        // Get the block based on the index.
        let block_index = Self::block_for_index(index);

        // Check if the index is out of range, if it is it has to 
        // expand to be able to contain the index.
        if block_index > self.blocks.len() - 1 {
            self.append_empty_blocks(block_index); 
        }
    
        // Set the value in the correct place.
        let corrected_index = Self::corrected_index(index); 
        self.blocks[block_index][corrected_index] = Some(item);
    }

    /// Returns the element associated with the key.
    /// 
    /// # Arguments
    /// 
    /// `index` - The index of the element to be returned.
    pub fn get(&self, index: usize) -> &Option<T> {
        // Get the bloc based on the index.
        let block_index = Self::block_for_index(index);
        let corrected_index = Self::corrected_index(index);

        // If the index is out of the blocks return none.
        if block_index > self.blocks_len() {
            return &None;
        }

        // Return a reference to the element.
        &self.blocks[block_index][corrected_index] 
    }
}

/// Provides implementations used to handle the memory.
impl<T, const N: usize> BlockVec<T, N> { 
    /// Forces the vector to creare N number of new blocks.
    /// 
    /// # Arguments
    /// 
    /// `num_of_blocks` - The number of blocks to expand.
    pub fn append_empty_blocks(&mut self, num_of_blocks: usize) {
        // Create all the new blocks.
        for _ in 0..num_of_blocks {
            self.blocks.push(
                iter::repeat_with(|| None)
                    .take(N)
                    .collect()
            );
        } 
    }
}

/// Provieds helpful functions.
impl<T, const N: usize> BlockVec<T, N> {
    /// Returns the block for the given index.
    ///
    /// # Arguments
    ///
    /// `index` - The global index.
    fn block_for_index(index: usize) -> usize {
        let float_index: f64 = index as f64;
        let float_n: f64 = N as f64;

        // Calculate in which block the index fall.
        // -1 due human's maths.
        (float_index / float_n).floor() as usize
    }

    /// Returns the relative index.
    ///
    /// # Arguments
    ///
    /// `index` - The index to be corrected.
    fn corrected_index(index: usize) -> usize {
        // Substract the block base from the index to get the correct position.
        index - (Self::block_for_index(index) * N)
    }

    /// Returns the number of elements in the vector.
    pub fn len(&self) -> usize {
        self.number_of_items
    }

    /// Returns the number of blocks.
    pub fn blocks_len(&self) -> usize {
        self.blocks.len()
    }
}

/// Syncs the vecs memory to the largest one.
/// 
/// # Arguments
/// 
/// `vecs` - The vectors to be sync.
pub fn sync_mem_to_biggest<
    T,
    const N: usize
>(vecs: Vec<&mut BlockVec<T, N>>) {
    // Contains the position of the element which contains the biggest
    // size.
    let mut biggest: usize = 0;
    
    // Search for the biggest.
    for block_vec in vecs.iter() {
        // Check if the current temp is smaller than the new value.
        if biggest < block_vec.blocks_len() {
            biggest = block_vec.blocks_len();
            continue;
        }
    }

    // Expand the vectors to have the corrent number of blocks.
    for block_vec in vecs {
        block_vec.append_empty_blocks(
            biggest - block_vec.blocks_len()
        );
    }
}

#[test]
fn creation() {
    let vec = BlockVec::<i32, 100>::new();
    assert_eq!(vec.blocks_len(), 1); 
    assert_eq!(vec.len(), 0);
}

#[test]
fn set_get_element() {
    let mut vec = BlockVec::<String, 100>::new();
    vec.set("A".to_string(), 50);
    vec.set("B".to_string(), 3);
    vec.set("C".to_string(), 5);

    if let Some(value) = vec.get(50) {
        assert_eq!(value, &"A".to_string());
    }

    if let Some(value) = vec.get(3) {
        assert_eq!(value, &"B".to_string());
    }

    if let Some(value) = vec.get(5) {
        assert_eq!(value, &"C".to_string());
    }

    vec.set("Replaced".to_string(), 50);

    if let Some(value) = vec.get(50) {
        assert_eq!(value, &"Replaced".to_string());
    }
}

#[test]
fn check_blocks() {
    let mut vec = BlockVec::<String, 5>::new();
    vec.set("A".to_string(), 50);
    vec.set("B".to_string(), 3);
    vec.set("C".to_string(), 5);    
    vec.set("A".to_string(), 20);
    vec.set("B".to_string(), 30);
    vec.set("C".to_string(), 50);

    assert_eq!(vec.blocks_len(), 11);
}

#[test]
fn sync_mem() {
    let mut vec1 = BlockVec::<String, 10>::new();
    let mut vec2 = BlockVec::<String, 10>::new();
    let mut vec3 = BlockVec::<String, 10>::new();

    vec1.set("A".to_string(), 100);
    vec2.set("B".to_string(), 4);

    sync_mem_to_biggest(vec![&mut vec1, &mut vec2, &mut vec3]);

    assert_eq!(vec1.blocks_len(), vec1.blocks_len());
    assert_eq!(vec1.blocks_len(), vec3.blocks_len());
}