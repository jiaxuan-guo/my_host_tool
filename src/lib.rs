pub mod utils {
    pub fn str_to_vec_u8 (name: String) -> Vec<u8> {
        let name:String = name.to_lowercase();
        let labels: Vec<String> = name.split(".").map(String::from).collect(); //["www","baidu","com"]
        let mut labels_u8: Vec<Vec<u8>> = labels
        .iter()
        .map(|s| s.as_bytes().to_vec())
        .collect();
        // prepend length
        for label in &mut labels_u8 {
            label.insert(0, label.len() as u8);
        }
        let qname = labels_u8.concat();
        qname
    }
}