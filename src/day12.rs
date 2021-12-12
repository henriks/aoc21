use std::collections::HashMap;

fn descend<'a>(
    path: &mut Vec<&'a str>,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    result: &mut u32,
    block_override: bool,
) -> () {
    let &curr = path.last().unwrap();

    if curr == "end" {
        *result += 1;
    } else {
        graph.get(curr).unwrap().iter().for_each(|&next| {
            let small = next.chars().all(char::is_lowercase);
            let unblocked = !small || !path.iter().any(|&s| s == next);
            if unblocked || (block_override && next != "start") {
                path.push(next);
                descend(path, graph, result, block_override && unblocked);
                path.pop();
            }
        });
    }
}

pub fn run() -> std::io::Result<()> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut add_edge = |from, to| {
        graph.entry(from).or_insert_with(|| vec![]).push(to);
    };

    let data = std::fs::read_to_string("data/12.txt")?;
    data.lines().for_each(|l| {
        let (a, b) = l.split_once("-").unwrap();
        add_edge(a, b);
        add_edge(b, a);
    });

    let mut path = vec!["start"];
    let mut result = 0;

    descend(&mut path, &graph, &mut result, false);
    println!("puzzle 12.1 {}", result);

    path = vec!["start"];
    result = 0;

    descend(&mut path, &graph, &mut result, true);
    println!("puzzle 12.2 {}", result);

    Ok(())
}
