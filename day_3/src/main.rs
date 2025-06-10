const START_OF_MUL_FUNCTION: &str = "mul(";
const DO_COMPUTE_NUMBERS: &str = "do()";
const STOP_COMPUTING_NUMBERS: &str = "don't()";

fn main() {
    let input = include_str!("./input.txt");
    let cleaned_numbers = clean_numbers(&input);

    let numbers = extract_mul_results(&cleaned_numbers);

    let result: i32 = numbers.iter().sum();
    println!("Result: {}", result);
}

/// Removes newlines and returns a vector of chars.
fn clean_numbers(input: &str) -> Vec<char> {
    input.replace('\n', "").replace('\r', "").chars().collect()
}

/// Checks if there are enough characters left from start_index.
fn reached_the_end(cleaned_numbers: &[char], start_index: usize, required_length: usize) -> bool {
    start_index + required_length <= cleaned_numbers.len()
}

/// Checks if the current index is the start of a "mul(" function.
fn search_for_string(cleaned_numbers: &[char], index: usize, search_for: &str) -> bool {
    cleaned_numbers.get(index..index + search_for.len())
        == Some(search_for.chars().collect::<Vec<char>>().as_slice())
}

/// Extracts all multiplication results from the cleaned input.
fn extract_mul_results(cleaned_numbers: &[char]) -> Vec<i32> {
    let mut numbers = Vec::new();
    let mut i = 0;

    let mut compute_numbers = true;

    while i < cleaned_numbers.len() {
        if !reached_the_end(cleaned_numbers, i, 10) {
            break;
        }

        if search_for_string(cleaned_numbers, i, DO_COMPUTE_NUMBERS) {
            compute_numbers = true;
        }

        if search_for_string(cleaned_numbers, i, STOP_COMPUTING_NUMBERS) {
            compute_numbers = false;
            i += 1;
            continue;
        }

        if compute_numbers {
            if search_for_string(cleaned_numbers, i, START_OF_MUL_FUNCTION) {
                if let Some((result, next_index)) = parse_mul_at(cleaned_numbers, i) {
                    numbers.push(result);
                    i = next_index;
                    continue;
                }
            }
        }
        i += 1;
    }
    numbers
}

/// Parses a "mul(num1,num2)" at the given index, returns (result, next_index) if successful.
fn parse_mul_at(cleaned_numbers: &[char], start: usize) -> Option<(i32, usize)> {
    let mut j = start + 4;
    let max_j = std::cmp::min(j + 8, cleaned_numbers.len());

    // Find comma
    while j < max_j && cleaned_numbers[j] != ',' {
        if !cleaned_numbers[j].is_digit(10) {
            return None;
        }
        j += 1;
    }
    if j >= max_j || cleaned_numbers[j] != ',' {
        return None;
    }

    // Find closing parenthesis
    let mut k = j + 1;
    let max_k = std::cmp::min(k + 8, cleaned_numbers.len());
    while k < max_k && cleaned_numbers[k] != ')' {
        if !cleaned_numbers[k].is_digit(10) {
            return None;
        }
        k += 1;
    }
    if k >= max_k || cleaned_numbers[k] != ')' {
        return None;
    }

    let number = cleaned_numbers[start + 4..j].iter().collect::<String>();
    let number2 = cleaned_numbers[j + 1..k].iter().collect::<String>();
    match (number.parse::<i32>(), number2.parse::<i32>()) {
        (Ok(n1), Ok(n2)) => Some((n1 * n2, k + 1)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_numbers_function() {
        let input = "mul(2,3)\n\nmul(4,5)";
        let cleaned_numbers = clean_numbers(input);
        assert_eq!(
            cleaned_numbers,
            vec!['m', 'u', 'l', '(', '2', ',', '3', ')', 'm', 'u', 'l', '(', '4', ',', '5', ')']
        );
    }

    #[test]
    fn test_cleaned_numbers() {
        let input = "mul(2,3)mul(4,5)";
        let cleaned_numbers: Vec<char> = input.chars().collect();
        assert_eq!(
            cleaned_numbers,
            vec!['m', 'u', 'l', '(', '2', ',', '3', ')', 'm', 'u', 'l', '(', '4', ',', '5', ')']
        );
    }

    #[test]
    fn test_reached_the_end() {
        let cleaned_numbers = vec!['m', 'u', 'l', '(', '2', ',', '3', ')'];
        assert!(reached_the_end(&cleaned_numbers, 0, 2));
        assert!(!reached_the_end(&cleaned_numbers, 0, 10));
        assert!(!reached_the_end(&cleaned_numbers, 7, 2));
    }

    #[test]
    fn test_extract_mul_results() {
        let cleaned_numbers = clean_numbers("mul(2,3)mul(4,5)");
        let results = extract_mul_results(&cleaned_numbers);
        assert_eq!(results, vec![6, 20]);
    }
}
