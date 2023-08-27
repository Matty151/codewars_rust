pub fn main() {
    for floor in tower_builder(1) {
        println!("{}", floor);
    }
}

fn tower_builder(n_floors: usize) -> Vec<String> {
    let mut tower = Vec::new();
    let max_blocks = (n_floors - 1) * 2 + 1;

    for floor in 0..n_floors {
        let n_blocks = floor * 2 + 1;

        let mut blocks: Vec<&str> = std::iter::repeat(" ").take(max_blocks).collect();

        for i in 0..n_blocks {
            blocks[i + ((max_blocks - n_blocks) / 2)] = "*";
        }

        tower.push(blocks.join(""));
    }

    tower
}
