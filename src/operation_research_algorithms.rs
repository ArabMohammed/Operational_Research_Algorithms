use rand::prelude::*;

/********************** Travel salesman problem  ****************/
/****************************************************************/
/*
Given a list of cities and the distance between each pair of cities, what is the 
shortest possible route that visits each city exactly once and returns to the 
starting city?
Ex: give a group of cities : [A,B,C,D] , the solutionmust ensure that : 
- Start at one city (e.g., A)
- Visit each city once
- Return to A
- Minimize the total travel distance

For n cities, the number of possible tours is roughly:
    ===> (n−1)!/2

This explosion of possibilities makes TSP a famous **NP-hard problem in 
Computational Complexity Theory.
**/
////////////////////Helper functions ///////////////////////////////////
// Compute the cost of a path 
fn compute_cost(path: &Vec<usize>, adj: &Vec<Vec<i32>>) -> i32 {
    let mut cost = 0;
    for i in 0..path.len() - 1 {
        cost += adj[path[i]][path[i + 1]];
    }
    cost
}
// Get the nearest neighbor of a city 
fn first_min(adj: &Vec<Vec<i32>>, i: usize) -> i32 {
    let mut min = i32::MAX;
    for j in 0..adj.len() {
        if i != j && adj[i][j] < min {
            min = adj[i][j];
        }
    }
    min
}
// Get the second nearest neighbor of a city 
fn second_min(adj: &Vec<Vec<i32>>, i: usize) -> i32 {
    let mut first = i32::MAX;
    let mut second = i32::MAX;

    for j in 0..adj.len() {
        if i == j {
            continue;
        }

        if adj[i][j] <= first {
            second = first;
            first = adj[i][j];
        } else if adj[i][j] < second {
            second = adj[i][j];
        }
    }

    second
}

pub fn generate_cities_matrix(count:usize) -> Vec<Vec<i32>> {
    let mut rng = rand::rng(); 
    let mut matrix:Vec<Vec<i32>> = vec![];
    for i in 0..count{
        let mut nums: Vec<i32> = (5..50).collect();
        nums.shuffle(&mut rng);
        // And take a random pick (yes, we didn't need to shuffle first!):
        let mut row : Vec<i32> = vec![];
        for j in 0..count{
            let val = nums.choose(&mut rng).unwrap().clone();
            row.push(val);
        }
        matrix.push(row)
    }
    matrix
}

////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////
// backtracking : 
// we explore all possible solution and take the one with lowest cost 
fn tsp_backtrack_req(
    adj: &Vec<Vec<i32>>,
    visited: &mut Vec<bool>,
    curr_pos: usize,
    count: usize,
    cost: i32,
    best_cost: &mut i32,
    start: usize,
) {
    let n = adj.len();

    // All cities visited
    if count == n && adj[curr_pos][start] > 0 {
        let total_cost = cost + adj[curr_pos][start];
        *best_cost = (*best_cost).min(total_cost);
        return;
    }

    for city in 0..n {
        if !visited[city] && adj[curr_pos][city] > 0 {
            visited[city] = true;

            tsp_backtrack_req(
                adj,
                visited,
                city,
                count + 1,
                cost + adj[curr_pos][city],
                best_cost,
                start,
            );

            // backtrack
            visited[city] = false;
        }
    }
}

pub fn tsp_backtrack(adj: &Vec<Vec<i32>>) -> i32{
    let n = adj.len();
    let mut visited = vec![false; n];
    visited[0] = true;
    let mut best_cost = i32::MAX;
    tsp_backtrack_req(adj, &mut visited, 0, 1, 0, &mut best_cost, 0);
    best_cost
}

////////////////////////////////////////////////////////////////////////
// Branch & bound Approach : 
// - It is used to solve minimlalization problems. 
// 1- Computes a lower bound for partial tours.
// 2- Prunes branches whose bound exceeds the current best solution.
// 3- Recursively explores promising paths.
fn tsp_branch_bound_req(
    adj: &Vec<Vec<i32>>,
    //lower bound for completing the tour from here.
    curr_bound: i32,
    //sum of distances along the current partial path.
    curr_weight: i32,
    //number of cities in the path.
    level: usize,
    curr_path: &mut Vec<usize>,
    //keeps track of visited cities.
    visited: &mut Vec<bool>,
    //best solution found so far.
    final_res: &mut i32,
    //path corresponding to final_res
    final_path: &mut Vec<usize>,
) {
    let n = adj.len();
    //we have traveled through all cities 
    //We complete the tour by returning to the start.
    if level == n {
        if adj[curr_path[level - 1]][curr_path[0]] != 0 {
            let curr_res = curr_weight + adj[curr_path[level - 1]][curr_path[0]];
            //Update final_res if this tour is better than current best.
            if curr_res < *final_res {
                *final_res = curr_res;
                final_path.clear();
                final_path.extend(curr_path.iter());
                final_path.push(curr_path[0]);
            }
        }
        return;
    }

    for i in 0..n {
        //For each unvisited city, we consider adding it next.
        if adj[curr_path[level - 1]][i] != 0 && !visited[i] {
            let mut curr_bound_new = curr_bound;
            let curr_weight_new = curr_weight + adj[curr_path[level - 1]][i];
            /*
            each city must have exactly 2 edges in the final tour : 
                - So if No edges chosen yet we subract the first_min
                - Else if One edge already chosen we subtract the second_min
            **/
            if level == 1 {
                curr_bound_new -= (first_min(adj, curr_path[level - 1]) + first_min(adj, i)) / 2;
            } else {
                curr_bound_new -= (second_min(adj, curr_path[level - 1]) + first_min(adj, i)) / 2;
            }
            //Prune if curr_bound + curr_weight >= final_res else recurse 
            if curr_bound_new + curr_weight_new < *final_res {
                curr_path[level] = i;
                visited[i] = true;
                tsp_branch_bound_req(
                    adj,
                    curr_bound_new,
                    curr_weight_new,
                    level + 1,
                    curr_path,
                    visited,
                    final_res,
                    final_path,
                );
            }

            visited[i] = false;
            curr_path[level] = usize::MAX;
        }
    }
}

pub fn tsp_branch_bound(adj: &Vec<Vec<i32>>) {
    let n = adj.len();
    let mut curr_path = vec![usize::MAX; n];
    let mut visited = vec![false; n];

    let mut curr_bound = 0;
    for i in 0..n {
        curr_bound += first_min(&adj, i) + second_min(&adj, i);
    }
    curr_bound = (curr_bound + 1) / 2;

    let mut final_res = i32::MAX;
    let mut final_path = Vec::new();

    curr_path[0] = 0;
    visited[0] = true;

    tsp_branch_bound_req(
        adj,
        curr_bound,
        0,
        1,
        &mut curr_path,
        &mut visited,
        &mut final_res,
        &mut final_path,
    );

    println!("Minimum cost: {}", final_res);
    //println!("Path: {:?}", final_path);
}

///////////////////////////////////////////////////////////////////////

// Nearest neighbor Approach (Greedy) : 
// For each city we take the nearest neighbor after checking that 
// it has not already been visited 
pub fn tsp_nearest_neighbor(adj: &Vec<Vec<i32>>) -> (i32, Vec<usize>) {
    let n = adj.len();
    let mut visited = vec![false; n];
    let mut path = Vec::new();

    let mut current = 0;
    let mut total_cost = 0;

    visited[current] = true;
    path.push(current);
    for _ in 1..n {
        let mut nearest = None;
        let mut min_dist = i32::MAX;
        for city in 0..n {
            if !visited[city] && adj[current][city] < min_dist {
                min_dist = adj[current][city];
                nearest = Some(city);
            }
        }

        let next = nearest.expect("Graph must be complete");
        visited[next] = true;
        path.push(next);
        total_cost += min_dist;
        current = next;
    }
    // return to initial city
    total_cost += adj[current][path[0]];
    path.push(path[0]);

    let mut best_path = path.clone();
    let mut best_cost = total_cost;
    (best_cost, best_path)
}
///////////////////////////////////////////////////////////////////////

// 2-Opt Approach (Greedy) : 
// Loop very cities in the initial path and swap edges(cities)
// Update path if the new cost is better 
pub fn tsp_two_opt(mut path: Vec<usize>, adj: &Vec<Vec<i32>>) -> (i32, Vec<usize>) {
    let n = path.len() - 1; // last == first
    let mut improved = true;
    /*
    nb_cities = 7;
    path = [0,3,2,5,6,4,1,0]
              |       |  
    i = 1 
    j = 5
    path[i..=j].reverse();
    path = [0,4,6,5,2,3,1,0]
              |       |
    ****/
    while improved {
        improved = false;

        for i in 1..n - 1 {
            for j in i + 1..n {
                let mut new_path = path.clone();

                //reverse segment
                new_path[i..=j].reverse();

                let new_cost = compute_cost(&new_path, adj);
                let old_cost = compute_cost(&path, adj);

                if new_cost < old_cost {
                    path = new_path;
                    improved = true;
                }
            }
        }
    }

    let best_cost = compute_cost(&path, adj);
    (best_cost, path)
}
////////////////////////////////////////////////////////////////////////

// Simulated annealing 
// Neighbor Generation algorithm : 
fn generate_neighbor(path : &Vec<usize>, rng:&mut ThreadRng) -> Vec<usize> {
    let n = path.len()-1;
    let mut new_path = path.clone();
    let i = rng.random_range(1..n);
    let j = rng.random_range(1..n);
    let (start,end) = if i < j {(i,j)} else {(j,i)};
    new_path[start..=end].reverse();
    new_path
}

pub fn tsp_simulated_annealing(adj:&Vec<Vec<i32>>) -> (i32,Vec<usize>){
    let n = adj.len();
    let mut rng = rand::rng();
    let val : f64 = rng.random();
    let arr1: [f32; 32] = rng.random();
    // Initial solution (simple path 0-->1--> ... >0)
    let mut current_path: Vec<usize> = (0..n).collect();
    current_path.push(0);
    let mut current_cost = compute_cost(&current_path, adj);
    let mut best_path = current_path.clone();
    let mut best_cost = current_cost;

    // Annealing params 
    let mut temperature = 2000.0;
    let mut cooling_rate = 0.998;
    let min_tempature = 1e-3;

    while temperature > min_tempature {
        // 1) Generate neighbor 
        let new_path = generate_neighbor(&current_path, &mut rng);
        let new_cost = compute_cost(&new_path, adj);
        let delta = new_cost - current_cost; 

        if delta < 0 || rng.random::<f64>() < (-delta as f64 / temperature).exp() {
            current_path = new_path;
            current_cost= new_cost;

            // update best 
            if current_cost < best_cost {
                best_cost = current_cost;
                best_path = current_path.clone();
            }
        } 
        // Cooling 
        temperature*=cooling_rate;
    }

    (best_cost,best_path)
}
////////////////////////////////////////////////////////////////////////

// Ant Colony Optimization (ACO): 
/**
    It is based on : 
        - Ants randomly explore paths. 
        - They leave pheromone on the ground. 
        - Shorted paths get more pheromone over time.
        - Other ants are more likely to follow thos paths.
****/
fn choose_next(
    current: usize,
    visited: &Vec<bool>,
    tau: &Vec<Vec<f64>>, //pheromone matrix
    adj: &Vec<Vec<i32>>,
    alpha: f64,
    beta: f64,
    rng: &mut ThreadRng,
) -> usize {
    let n = adj.len();
    let mut probs = vec![0.0; n];
    let mut sum = 0.0;

    for j in 0..n {
        // For each possible next city j:
        // tau[current][j] → how good this edge was historically
        // 1 / dist[current][j] → closer cities are better
        if !visited[j] {
            let pheromone = tau[current][j].powf(alpha);
            let heuristic = (1.0 / adj[current][j] as f64).powf(beta);
            probs[j] = pheromone * heuristic;
            sum += probs[j];
        }
    }

    // roulette wheel selection 
    // 3 cities with probs : 
    // prob[A] = 0.5 
    // prob[B] = 0.2 
    // prob[C] = 0.3 
    // if we have pick = 0.65 , then : 
    // pick -= prob[A] ==> 0.15 
    // pick -= prob[B] ==> -0.5

    // pick a random number between 0 ---> total_probability
    let mut pick = rng.random::<f64>() * sum;

    for j in 0..n {
        if !visited[j] {
            pick -= probs[j];
            if pick <= 0.0 {
                return j;
            }
        }
    }

    // fallback
    (0..n).find(|&j| !visited[j]).unwrap()
}

pub fn tsp_aco(adj: &Vec<Vec<i32>>) -> (i32, Vec<usize>) {
    let n = adj.len();
    let mut rng = rand::rng();

    let num_ants = n;
    let iterations = 10000;

    let alpha = 1.0; // pheromone importance
    let beta = 5.0;  // distance importance
    let rho = 0.5;   // evaporation rate
    let q = 100.0;   // pheromone deposit factor

    // Initialize pheromone matrix
    let mut tau = vec![vec![1.0; n]; n];

    let mut best_path = Vec::new();
    let mut best_cost = i32::MAX;

    for _ in 0..iterations {
        let mut all_paths = Vec::new();

        // ===== Ants build solutions =====
        for _ in 0..num_ants {
            let mut visited = vec![false; n];
            let mut path = Vec::new();

            let mut current = 0;
            visited[current] = true;
            path.push(current);

            for _ in 1..n {
                let next = choose_next(
                    current,
                    &visited,
                    &tau,
                    adj,
                    alpha,
                    beta,
                    &mut rng,
                );

                path.push(next);
                visited[next] = true;
                current = next;
            }

            path.push(0); // return to start

            let cost = compute_cost(&path, adj);

            if cost < best_cost {
                best_cost = cost;
                best_path = path.clone();
            }

            all_paths.push((path, cost));
        }

        //===> Evaporation 
        for i in 0..n {
            for j in 0..n {
                tau[i][j] *= 1.0 - rho;
            }
        }

        //===> Reinforcement 
        for (path, cost) in all_paths {
            let contribution = q / cost as f64;

            for i in 0..path.len() - 1 {
                let a = path[i];
                let b = path[i + 1];

                tau[a][b] += contribution;
                tau[b][a] += contribution;
            }
        }
    }

    (best_cost, best_path)
}
////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////
