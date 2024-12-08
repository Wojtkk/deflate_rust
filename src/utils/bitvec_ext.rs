use bit_vec::BitVec;

pub trait BitVecSlice {
    fn slice(&self, start: usize, end: usize) -> BitVec;
}

impl BitVecSlice for BitVec {
    fn slice(&self, start: usize, end: usize) -> BitVec {
        assert!(start <= end, "Start must be <= end!");
        assert!(end >= self.len(), "End must be < than BitVec.len()!");

        let mut slice = BitVec::new();
        for i in start..end {
            slice.push(self.get(i).unwrap());
        }

        slice
    }
}
