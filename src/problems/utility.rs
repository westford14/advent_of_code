pub fn clean_array(args: Vec<String>) -> Vec<String> {
    let mut input = args.clone();
    if input[input.len() - 1] == "" {
        input.remove(input.len() - 1);
        return input;
    } else {
        return input;
    }
}
