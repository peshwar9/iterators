
fn main() {
    let v = vec!["text1".to_string(),"text2".to_string()];
    
    println!("{:?}",flatten_string(v.clone().into_iter()));
    println!("{:?}", flatten_string2(v.clone().into_iter()).collect::<Vec<char>>());
    
}

fn flatten_string(ss: impl Iterator<Item=String>) -> Vec<char> {
    ss.flat_map(|s| s.chars().collect::<Vec<_>>()).collect()
}

fn flatten_string2(ss: impl Iterator<Item=String>) -> impl Iterator<Item=char> {
    ss.flat_map(|s| s.chars().collect::<Vec<_>>())
}
