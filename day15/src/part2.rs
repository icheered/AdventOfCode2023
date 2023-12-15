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
    input
        .chars()
        .fold(0, |acc, c| ((acc + c as u16) * 17) % 256)
}

fn parse_input(input: &str) -> Vec<Instruction> {
    // Input string: pnb=7,zxz-
    // Split on comma
    // If ends in -, instruction(characters, hash(characters before -), false, 0)
    // Else split on = instruction(characters, hash(characters before =), false, number after =)

    // abc=5,def-

    input
        .split(',')
        .map(|item| {
            let (label, lens) = if item.ends_with('-') {
                (item[..item.len() - 1].to_string(), 0)
            } else {
                let parts: Vec<&str> = item.split('=').collect();
                (
                    parts[0].to_string(),
                    parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0),
                )
            };

            Instruction {
                label: label.clone(),
                hash: get_hash(label.as_str()),
                lens,
            }
        })
        .collect::<Vec<Instruction>>()
}

#[allow(unused_variables)]
pub fn solve(input: &str) -> i64 {
    let instructions = parse_input(input);

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

    // Calculate focus power
    boxes
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
        })
}
