use std::collections::HashSet;

use rand::prelude::*; 

////////////////////////////////////////////////////////////////////////////////
/// best fit heuristic
pub fn bin_packing_first_fit(objects_sizes: &Vec<i32> , bin_capacity : i32) -> Vec<Vec<i32>> {

    let mut bins_content : Vec<Vec<i32>> = vec![];
    bins_content.push(vec![objects_sizes[0]]);
    for &object in &objects_sizes[1..] {
        let mut new_bin = true;
        let mut bin_remaining_space = bin_capacity;
        let mut selected_bin_index = 0;
        let mut i =0;
        for bin in &bins_content{
            let content_size : i32 = bin.iter().sum();
            if content_size + object <= bin_capacity {
                new_bin=false;
                if bin_remaining_space > (bin_capacity-content_size-object) {
                    selected_bin_index = i;
                    bin_remaining_space = bin_capacity-content_size-object;
                }
            }
            i+=1;
        }
        if new_bin{
            bins_content.push(vec![object])
        }else{
            bins_content[selected_bin_index].push(object);
        }
    }
    bins_content
}

////////////////////////////////////////////////////////////////////////////////
/// Backtracking 
// For each item, place it in every possible bin(if it fits)
// if no bin fits --> create a new bin 
fn bin_packing_backtracking_recursive(items: &Vec<i32>, bins:&mut Vec<Vec<i32>>, capacity: i32, 
    index: usize, best: &mut usize, best_solution: &mut Vec<Vec<i32>>) {
    // All items placed
    if index == items.len(){
        if bins.len() < *best{
            *best = bins.len();
            *best_solution = bins.clone();
        }
    }
    
    if bins.len() >= *best{
        return;
    }
    let item = items[index];
    for i in 0..bins.len(){
        let current_sum : i32 = bins[i].iter().sum();
        if current_sum + item <= capacity {
            bins[i].push(item);
            
            bin_packing_backtracking_recursive(items, bins, capacity, index + 1, best, best_solution);

            bins[i].pop(); // backtrack
        }
    }
    
    // Create new bin
    bins.push(vec![item]);
    bin_packing_backtracking_recursive(items, bins, capacity, index + 1, best, best_solution);
    bins.pop(); // backtrack
}

pub fn bin_packing_backtracking(items: &Vec<i32>, capacity: i32) -> (usize,Vec<Vec<i32>>) {
    let mut items = items.clone();
    // Start fisrt with large size objects to reduce the number of branches 
    items.sort_by(|a,b|b.cmp(a));

    let mut best = items.len(); // worst case: 1 item per bin
    let mut bins : Vec<Vec<i32>> = Vec::new();
    let mut best_solution = Vec::new();

    bin_packing_backtracking_recursive(&items, &mut bins, capacity, 0, &mut best, &mut best_solution);

    (best,best_solution)
}

////////////////////////////////////////////////////////////////////////////////
// Branch&Bound 
fn bin_packing_branch_bound_recursive(items: &Vec<i32>, bins:&mut Vec<Vec<i32>>, capacity: i32, 
    index: usize,loads : &mut Vec<i32>,best: &mut usize, best_solution: &mut Vec<Vec<i32>>) {
    // All items placed
    if index == items.len(){
        if bins.len() < *best{
            *best = bins.len();
            *best_solution = bins.clone();
        }
    }
    // Prune if already worse 
    if bins.len() >= *best{
        return;
    }

    //  Lower bound
    let remaining_sum: i32 = items[index..].iter().sum();
    let min_additional_bins = (remaining_sum + capacity - 1) / capacity;

    if bins.len() + min_additional_bins as usize >= *best {
        return;
    }
    let item = items[index];

    let mut used_loads = HashSet::new();
    for i in 0..bins.len() {
        if loads[i] + item <= capacity {
            // Avoid duplicate states (same load bins)
            if used_loads.contains(&loads[i]) {
                continue;
            }
            used_loads.insert(loads[i]);

            bins[i].push(item);
            loads[i] += item;

            bin_packing_branch_bound_recursive(
                items,
                bins,
                capacity ,
                index+1,
                loads,
                best,
                best_solution,
            );

            // backtrack
            bins[i].pop();
            loads[i] -= item;
        }
    }

    // ===== Try new bin =====
    bins.push(vec![item]);
    loads.push(item);

    bin_packing_branch_bound_recursive(
        items,
        bins,
        capacity ,
        index+1,
        loads,
        best,
        best_solution,
    );

    bins.pop();
    loads.pop();
}

pub fn bin_packing_branch_bound(items: &Vec<i32>, capacity: i32) -> (usize,Vec<Vec<i32>>) {
    let mut items = items.clone();
    // Start fisrt with large size objects to reduce the number of branches 
    items.sort_by(|a,b|b.cmp(a));

    let mut best = items.len(); // worst case: 1 item per bin
    let mut bins : Vec<Vec<i32>> = Vec::new();
    let mut loads: Vec<i32> = Vec::new();
    let mut best_solution = Vec::new();

    bin_packing_branch_bound_recursive(&items, &mut bins, capacity, 0,&mut loads ,&mut best, &mut best_solution);

    (best,best_solution)
}  

////////////////////////////////////////////////////////////////////////////////
// Simulated annealing 
/// Compute cost: prioritize fewer bins, then less unused space
fn compute_cost(bins: &Vec<Vec<i32>>, bin_sums: &Vec<i32>, capacity: i32) -> i64 {
    let unused: i32 = bin_sums.iter().map(|s| capacity - s).sum();
    (bins.len() as i64) * 10_000 + unused as i64
}

/// Generate a neighbor solution
fn generate_neighbor(
    bins: &Vec<Vec<i32>>,
    bin_sums: &Vec<i32>,
    capacity: i32,
    rng: &mut ThreadRng,
) -> (Vec<Vec<i32>>, Vec<i32>) {
    let mut new_bins = bins.clone();
    let mut new_sums = bin_sums.clone();

    let action = rng.random_range(0..3);

    match action {
        // Swap items between two bins
        0 => {
            if new_bins.len() < 2 {
                return (new_bins, new_sums);
            }

            let b1 = rng.random_range(0..new_bins.len());
            let mut b2 = rng.random_range(0..new_bins.len());
            while b1 == b2 {
                b2 = rng.random_range(0..new_bins.len());
            }

            if new_bins[b1].is_empty() || new_bins[b2].is_empty() {
                return (new_bins, new_sums);
            }

            let i1 = rng.random_range(0..new_bins[b1].len());
            let i2 = rng.random_range(0..new_bins[b2].len());

            let item1 = new_bins[b1][i1];
            let item2 = new_bins[b2][i2];

            let new_sum_b1 = new_sums[b1] - item1 + item2;
            let new_sum_b2 = new_sums[b2] - item2 + item1;

            if new_sum_b1 <= capacity && new_sum_b2 <= capacity {
                new_bins[b1][i1] = item2;
                new_bins[b2][i2] = item1;

                new_sums[b1] = new_sum_b1;
                new_sums[b2] = new_sum_b2;
            }
        }

        // Move item from one bin to another
        1 => {
            if new_bins.len() < 2 {
                return (new_bins, new_sums);
            }

            let src = rng.random_range(0..new_bins.len());
            let mut dst = rng.random_range(0..new_bins.len());
            while src == dst {
                dst = rng.random_range(0..new_bins.len());
            }

            if new_bins[src].is_empty() {
                return (new_bins, new_sums);
            }

            let idx = rng.random_range(0..new_bins[src].len());
            let item = new_bins[src][idx];

            if new_sums[dst] + item <= capacity {
                // move
                new_bins[dst].push(item);
                new_sums[dst] += item;

                new_bins[src].swap_remove(idx);
                new_sums[src] -= item;

                // remove empty bin safely
                if new_bins[src].is_empty() {
                    new_bins.swap_remove(src);
                    new_sums.swap_remove(src);
                }
            }
        }
        
        // Split a bin and reinsert its items (VERY IMPORTANT)
        2 => {
            if new_bins.is_empty() {
                return (new_bins, new_sums);
            }

            // pick a random bin to destroy
            let b = rng.random_range(0..new_bins.len());

            // take all items
            let mut items = new_bins.swap_remove(b);
            new_sums.swap_remove(b);

            // shuffle items for randomness
            items.shuffle(rng);

            // try to reinsert items greedily
            for item in items {
                let mut placed = false;

                // try existing bins first
                for i in 0..new_bins.len() {
                    if new_sums[i] + item <= capacity {
                        new_bins[i].push(item);
                        new_sums[i] += item;
                        placed = true;
                        break;
                    }
                }

                // if not placed → create new bin
                if !placed {
                    new_bins.push(vec![item]);
                    new_sums.push(item);
                }
            }
        }

        _ => {}
    }

    (new_bins, new_sums)
}

/// Initial solution: First Fit
fn first_fit(items: &Vec<i32>, capacity: i32) -> (Vec<Vec<i32>>, Vec<i32>) {
    let mut bins: Vec<Vec<i32>> = Vec::new();
    let mut sums: Vec<i32> = Vec::new();

    for &item in items {
        let mut placed = false;

        for i in 0..bins.len() {
            if sums[i] + item <= capacity {
                bins[i].push(item);
                sums[i] += item;
                placed = true;
                break;
            }
        }

        if !placed {
            bins.push(vec![item]);
            sums.push(item);
        }
    }

    (bins, sums)
}

/// Simulated Annealing Solver
pub fn bin_packing_simulated_annealing(items: &Vec<i32>,capacity: i32,) -> (usize, Vec<Vec<i32>>) {
    let mut rng = rand::rng();

    // Initial solution
    let (mut current_bins, mut current_sums) = first_fit(items, capacity);
    let mut current_cost = compute_cost(&current_bins, &current_sums, capacity);

    let mut best_bins = current_bins.clone();
    let mut best_cost = current_cost;

    // SA parameters
    let mut temperature = 1000.0;
    let cooling_rate = 0.995;
    let min_temp = 1e-3;
    let iterations_per_temp = 50;

    while temperature > min_temp {
        for _ in 0..iterations_per_temp {
            let (new_bins, new_sums) =
                generate_neighbor(&current_bins, &current_sums, capacity, &mut rng);

            let new_cost = compute_cost(&new_bins, &new_sums, capacity);
            let delta = new_cost - current_cost;

            if delta < 0
                || rng.random::<f64>() < (-delta as f64 / temperature).exp()
            {
                current_bins = new_bins;
                current_sums = new_sums;
                current_cost = new_cost;

                if current_cost < best_cost {
                    best_cost = current_cost;
                    best_bins = current_bins.clone();
                }
            }
        }

        temperature *= cooling_rate;
    }

    (best_bins.len(), best_bins)
}

 