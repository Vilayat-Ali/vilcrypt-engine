pub struct cube {
    cube_string: Vec<Vec<Vec<char>>>,
    dimension: usize,
}

impl cube {
    // constructor
    pub fn new(data: String) -> Self {
        let mut dimension: usize = 3; // default
        while data.len() > (dimension*dimension*6) {
            dimension += 1;
        }
        cube {
            cube_string: vec![vec![vec!['#'; dimension]; dimension]; 6],
            dimension
        }
    }
    // mixing
    pub fn mix(&mut self, move_list: &str) -> Result<(), _> {
        Ok(())
    }
}
