use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Instruction {
    label: String,
    hash: u16,
    lens: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Lens {
    label: String,
    lens: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Box {
    lenses: Vec<Lens>,
}

fn get_hash(input: &str) -> u16 {
    input.chars().fold(0, |acc, c| {
        ((acc.wrapping_add(c as u16)).wrapping_mul(17)) & 0xFF
    })
}

fn parse_input(input: &str) -> Vec<Instruction> {
    // Input string: pnb=7,zxz-
    // Split on comma
    // If ends in -, instruction(characters, hash(characters before -), false, 0)
    // Else split on = instruction(characters, hash(characters before =), false, number after =)

    let parts: Vec<&str> = input.split(',').collect();
    let mut instructions = Vec::with_capacity(parts.len());

    for item in parts {
        let end_char = item.chars().last().unwrap_or_default();
        let (label, lens) = if end_char == '-' {
            (item[..item.len() - 1].to_string(), 0)
        } else {
            let split_at = item.find('=').unwrap_or(item.len());
            (
                item[..split_at].to_string(),
                item.get(split_at + 1..)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0),
            )
        };

        let hash = get_hash(&label);

        instructions.push(Instruction { label, hash, lens });
    }

    instructions
}

#[allow(unused_variables)]
pub fn solve(input: &str) -> i64 {
    // let t1 = Instant::now();
    let instructions = parse_input(input);
    // let t2 = Instant::now();

    // Create a vector of 255 boxes
    let mut boxes: Vec<Box> = (0..256).map(|_| Box { lenses: Vec::new() }).collect();

    for instruction in instructions {
        if instruction.lens == 0 {
            // Remove the lens with the given label if it is present in the box
            boxes[instruction.hash as usize]
                .lenses
                .retain(|l| l.label != instruction.label);

            // Move any remaining lenses as far forward in the box as they can go without changing their order, filling any space made by removing the indicated lens
            boxes[instruction.hash as usize].lenses = boxes[instruction.hash as usize]
                .lenses
                .iter()
                .enumerate()
                .map(|(index, lens)| Lens {
                    label: lens.label.clone(),
                    lens: lens.lens,
                })
                .collect();
        } else if boxes[instruction.hash as usize]
            .lenses
            .iter()
            .any(|l| l.label == instruction.label)
        {
            // Replace the lens with the new lens (update the lens value)
            boxes[instruction.hash as usize]
                .lenses
                .iter_mut()
                .find(|l| l.label == instruction.label)
                .map(|l| l.lens = instruction.lens);
        } else {
            // Add the lens to the box immediately behind any lenses already in the box
            boxes[instruction.hash as usize].lenses.push(Lens {
                label: instruction.label,
                lens: instruction.lens,
            });
        }
    }
    // let t3 = Instant::now();

    // Calculate focus power
    let focus_power = boxes
        .iter()
        .enumerate()
        .fold(0, |acc, (box_index, box_item)| {
            acc + box_item
                .lenses
                .iter()
                .enumerate()
                .fold(0, |acc, (lens_index, lens)| {
                    let focus_power =
                        lens.lens as i64 * (lens_index + 1) as i64 * (box_index + 1) as i64;
                    acc + focus_power
                })
        });

    let t4 = Instant::now();
    // println!("Parse input: {:?}", t2.duration_since(t1));
    // println!("Process input: {:?}", t3.duration_since(t2));
    // println!("Calculate focus power: {:?}", t4.duration_since(t3));

    focus_power
}
