pub fn build_proverb(list: Vec<&str>) -> String {

    let size = list.len();
    if size == 0 {
        return "".into()
    }

    let mut wants = vec![];

    for (i, item) in list.iter().enumerate() {
        if i + 1 == size {
            break
        }

        wants.push(format!("For want of a {} the {} was lost.", item, list.get(i + 1).unwrap()));
    }

    wants.push(format!("And all for the want of a {}.", list.get(0).unwrap()));
    wants.join("\n")
}
