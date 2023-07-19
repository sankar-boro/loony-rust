mod algorithms;

use std::sync::Arc;
use algorithms::database::mvcc::MVCC;

fn main() {
    let mvcc = Arc::new(MVCC::new());
    let mut transaction = mvcc.begin_transaction();
    mvcc.write(&mut transaction, "key1".to_owned(), "value1".to_owned());
    mvcc.write(&mut transaction, "key2".to_owned(), "value2".to_owned());

    let value1 = mvcc.read(&mut transaction, "key1");
    let value2 = mvcc.read(&mut transaction, "key2");

    let committed = mvcc.commit(transaction);
    
    if let Some((_, v1)) = value1 {
        println!("Value of key1: {}", v1);
    }
    if let Some((_, v2)) = value2 {
        println!("Value of key2: {}", v2);
    }

    if committed {
        println!("Transaction committed successfully.");
    } else {
        println!("Transaction aborted due to conflicts.");
    }
}