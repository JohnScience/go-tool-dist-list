fn main() {
    let targets = go_tool_dist_list::from_cli().unwrap();
    for (i,target) in targets.iter().unwrap().enumerate() {
        println!("{i}. {target}");
    }
}