/// What should the type of _function be?
pub fn map<T, U, F>(input: Vec<T>, mut function: F) -> Vec<U>
    where F: FnMut(T) -> U {
    let mut output = Vec::new();
    for i in input {
        output.push(function(i));
    }
    output
}
