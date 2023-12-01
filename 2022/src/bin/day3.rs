use anyhow::Result;

fn main() -> Result<()> {
    let collection: Vec<&str> = include_str!("./day3_input.test").lines().collect();

    let groups = collection.chunks(3);

    let total: u32 = groups
        .map(move |a| {
            let mut group = a.iter();
            let x = group.next().unwrap();
            let y = group.next().unwrap();
            let z = group.next().unwrap();
            let mut similar = 'a';
            for c in x.chars() {
                for d in y.chars() {
                    for e in z.chars() {
                        if c == d && d == e {
                            similar = c;
                            break;
                        }
                    }
                }
            }
            let value = similar as u32;
            if value >= 97 {
                return value - 96;
            }
            return value - 38;
        })
        .sum();

    println!("{:?}", total);

    Ok(())
}
