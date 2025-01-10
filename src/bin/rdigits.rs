use proconio::{fastout, input, marker::Chars};

fn can_erase(original_slice: &[u32], to_remove_count: &[i32]) -> bool {
    let mut slice_count = [0; 10];
    for digit in original_slice {
        slice_count[*digit as usize] += 1;
    }

    let mut can_erase = true;
    for digit in 1..=9 {
        can_erase = can_erase && slice_count[digit] <= to_remove_count[digit];
    }
    can_erase
}

#[fastout]
fn main() {
    input! { original: Chars, to_remove: Chars };
    let original: Vec<u32> = original.iter().map(|d| d.to_digit(10).unwrap()).collect();
    let to_remove: Vec<u32> = to_remove.iter().map(|d| d.to_digit(10).unwrap()).collect();

    let mut original_count = [0; 10];
    for digit in &original {
        original_count[*digit as usize] += 1;
    }

    let mut to_remove_count = [0; 10];
    for digit in &to_remove {
        to_remove_count[*digit as usize] += 1;
    }

    let mut remaining = &original[..];
    for _ in 0..(original.len() - to_remove.len()) {
        for try_digit in (1..=9).rev() {
            let Some(pos) = remaining.iter().position(|d| *d == try_digit) else {
                continue;
            };

            if can_erase(&remaining[..pos], &to_remove_count)
                && original_count[try_digit as usize] > to_remove_count[try_digit as usize]
            {
                print!("{}", remaining[pos]);

                for digit in &remaining[..pos] {
                    original_count[*digit as usize] -= 1;
                    to_remove_count[*digit as usize] -= 1;
                }
                original_count[remaining[pos] as usize] -= 1;
                remaining = &remaining[pos + 1..];
                break;
            }
        }
    }
    println!();
}
