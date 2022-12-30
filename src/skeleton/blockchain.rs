use crate::skeleton::block::Block;
use rocksdb::{DB, Options};

struct Blockchain
{
    db: DB,
}

impl Blockchain
{
    fn new(path: &str) -> Blockchain
    {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        let db = DB::open(&opts, path).unwrap();

        Blockchain { db }
    }
}
