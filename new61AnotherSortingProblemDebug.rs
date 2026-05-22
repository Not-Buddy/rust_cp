fn solve_debug(vec: Vec<i32>) {
    let num = vec.len();
    if num <= 1 {
        println!("Array is too small, obviously YES.");
        return;
    }

    let mut max_diff = 0;
    for i in 0..num - 1 {
        if vec[i] > vec[i+1] {
            max_diff = max_diff.max(vec[i] - vec[i+1]);
        }
    }

    let k = max_diff;
    println!("Target Array: {:?}", vec);
    println!("Calculated k (maximum drop): {}", k);
    println!("==================================================");

    if k == 0 {
        println!("Array is already sorted! k = 0. YES");
        return;
    }

    let mut can_0 = true;
    let mut can_1 = true;

    println!("Index 0 (Value: {})", vec[0]);
    println!("  can_0 (keep as {}): true", vec[0]);
    println!("  can_1 (boost to {}): true", vec[0] + k);
    println!("--------------------------------------------------");

    for i in 1..num {
        let prev = vec[i-1];
        let curr = vec[i];
        
        println!("Index {} (Current Value: {}, Previous Value: {})", i, curr, prev);

        let path_0_to_0 = can_0 && (prev <= curr);
        let path_1_to_0 = can_1 && (prev + k <= curr);
        
        let new_0 = path_0_to_0 || path_1_to_0;

        println!("  Checking new_0 (Trying to keep current as {}):", curr);
        println!("    <- From prev left alone ({} <= {}): {}", prev, curr, path_0_to_0);
        println!("    <- From prev boosted    ({} <= {}): {}", prev + k, curr, path_1_to_0);
        println!("    >>> Result for can_0: {}", new_0);


        let path_0_to_1 = can_0; 
        let path_1_to_1 = can_1 && (prev <= curr); // Simplified from: prev + k <= curr + k

        let new_1 = path_0_to_1 || path_1_to_1;

        println!("  Checking new_1 (Trying to boost current to {}):", curr + k);
        println!("    <- From prev left alone ({} <= {}): {} (Always true if previous can_0 is true)", prev, curr + k, path_0_to_1);
        println!("    <- From prev boosted    ({} <= {}): {}", prev + k, curr + k, path_1_to_1);
        println!("    >>> Result for can_1: {}", new_1);

        can_0 = new_0;
        can_1 = new_1;
        
        println!("--------------------------------------------------");
        
        if !can_0 && !can_1 {
            println!("Both paths failed at index {}. The array cannot be sorted.", i);
            println!("Final Answer: NO");
            return;
        }
    }

    println!("Reached the end! can_0: {}, can_1: {}", can_0, can_1);
    println!("Final Answer: YES");
}

fn main() {
    let test_array = vec![4, 2, 5]; 
    solve_debug(test_array);
}