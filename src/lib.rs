#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate sha2;

use sha2::Digest;
use sha2::Sha256 as Hash;

#[derive(Debug,PartialEq,Eq,PartialOrd,Ord)]
pub struct Block {
    index: u64,
    previous_hash: Box<[u8]>,
    timestamp: ::std::time::SystemTime,
    data: Box<[u8]>,
    hash: Box<[u8]>,
}

impl Default for Block {
    fn default() -> Self {
       let mut b = Block {
            index: 0,
            previous_hash: Box::new([]),
            timestamp: ::std::time::UNIX_EPOCH,
            data: Box::new(b"initial data".to_owned()),
            hash: Box::new([]),
       };
       b.add_hash();
       b
    }
}

impl Block {
    fn hash(&self) -> Box<[u8]>
    {
        let mut s = Hash::default();
        s.input(format!("{}", self.index).as_bytes());
        s.input(&self.previous_hash[..]);
        s.input(format!("{}", self.timestamp.duration_since(::std::time::UNIX_EPOCH).unwrap().as_secs()).as_bytes());
        s.input(&self.data[..]);
        s.result().as_slice().to_owned().into_boxed_slice()
    }

    fn add_hash(&mut self)
    {
        self.hash = self.hash(); 
    }

    pub fn is_valid_next_block(&self, next: &Self) -> bool
    {
        self.index + 1 == next.index &&
            self.hash == next.previous_hash &&
            next.hash == next.hash()
    }

    pub fn mine(&self, data: Box<[u8]>) -> Self {
       let mut b = Block {
            index: self.index + 1,
            previous_hash: self.hash.clone(),
            timestamp: ::std::time::SystemTime::now(),
            data: data,
            hash: Box::new([]),
       };
       b.add_hash();
       b
    }
}

pub fn is_valid_chain(chain: &[Block])
    -> bool
{
    let l = chain.len();
    if l == 0 { return true; }

    if chain[l] != Block::default() {
        return false;
    }

    for w in chain.windows(2) {
        if !w[0].is_valid_next_block(&w[1]) {
            return false;
        }
    }
    
    true
}

#[derive(Debug,PartialEq,Eq,PartialOrd,Ord, Serialize, Deserialize)]
pub struct BlockArgs {
    body: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
