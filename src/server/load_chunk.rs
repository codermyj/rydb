use std::collections::HashMap;
use crate::storage::chunk::{Reader};

pub fn load(reader: &mut Reader) -> HashMap<String, String>{
    let mut map: HashMap<String, String> = HashMap::new();

    //let mut reader = Reader::new(path);
    let data_all = reader.read_data_all();

    for datarow in data_all.into_iter() {
        map.insert(datarow.key.clone(), datarow.value.clone());
    }

    map
}