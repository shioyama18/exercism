pub fn build_proverb(list: Vec<&str>) -> String {
    if list.is_empty() {
        String::new()
    } else {
        list.windows(2)
            .map(|w| format!("For want of a {} the {} was lost.", w[0], w[1]))
            .chain(std::iter::once(format!("And all for the want of a {}.", list[0])))
            .collect::<Vec<_>>()
            .join("\n") + &format!("\nAnd all for the want of a {}.", list[0])
    }
}
