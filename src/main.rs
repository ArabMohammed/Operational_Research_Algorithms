use std::u32;
use rand::prelude::*;
use std::time::Instant;
mod operation_research_algorithms;
enum Problems {
    TSP
}

fn main() {
    let problem = Problems::TSP;
    match problem {
        Problems::TSP=>{
            // Example dynamic table
            let mut adj = vec![
                vec![0, 10, 15, 20],
                vec![10, 0, 35, 25],
                vec![15, 35, 0, 30],
                vec![20, 25, 30, 0],
            ];
            let number_cities = 10;
            adj = operation_research_algorithms::generate_cities_matrix(number_cities);
            println!("===> cities matrix {:?}",adj);
            ///////////////////////////////////////////////////
            /*let mut start = Instant::now();
            operation_research_algorithms::tsp_backtrack(adj);
            let time_backtracking = start.elapsed();
            start = Instant::now(); 
            operation_research_algorithms::tsp_branch_bound(adj);
            let time_bnb = start.elapsed();
            println!("Backtracking time: {:?}", time_backtracking);
            println!("Branch & Bound time: {:?}", time_bnb);*/
            
            /*for n in 5..12 {
                let adj = generate_cities_matrix(n);

                println!("Cities: {}", n);

                let start = Instant::now();
                operation_research_algorithms::tsp_backtrack(&adj);
                println!("Backtracking: {:?}", start.elapsed());

                let start = Instant::now();
                operation_research_algorithms::tsp_branch_bound(&adj);
                println!("Branch&Bound: {:?}", start.elapsed());

                println!("------------------------------------------------");
            }*/
            let total_cost = operation_research_algorithms::tsp_backtrack(&adj);
            println!("======> backtracking results : ");
            println!("===> total_cost : {}",total_cost);
            ///////////////////////////////////////////////////////////////////////
            let (total_cost,path) = operation_research_algorithms::tsp_nearest_neighbor(&adj);
            println!("======> Nearest neighbor results : ");
            println!("===> total_cost : {}",total_cost);
            println!("===> path : {:?}",path);
            ///////////////////////////////////////////////////////////////////////
            let (total_cost,path) = operation_research_algorithms::tsp_two_opt(path,&adj);
            println!("======> 2-Opt optimization From Nearest neighbor Path Results: ");
            println!("===> total_cost : {}",total_cost);
            println!("===> path : {:?}",path);
            ///////////////////////////////////////////////////////////////////////
            let mut path: Vec<usize> = (0..number_cities).collect();
            path.push(0);
            let (total_cost,path) = operation_research_algorithms::tsp_two_opt(path,&adj);
            println!("======> 2-Opt optimization From Initial Path Results: ");
            println!("===> total_cost : {}",total_cost);
            println!("===> path : {:?}",path);
            ///////////////////////////////////////////////////////////////////////
            let (total_cost,path) = operation_research_algorithms::tsp_simulated_annealing(&adj);
            println!("======> Simulated annealing Results: ");
            println!("===> total_cost : {}",total_cost);
            println!("===> path : {:?}",path);
            ///////////////////////////////////////////////////////////////////////
            let (total_cost,path) = operation_research_algorithms::tsp_aco(&adj);
            println!("======> ACO Results: ");
            println!("===> total_cost : {}",total_cost);
            println!("===> path : {:?}",path);
        }
        _=>{
            //let val : usize = 10;
            //let res = operation_research_algorithms::fibonacci(val);
            //println!("fibonacci({}) ===> {}",val,res);
            //operation_research_algorithms::knapsack_daynamic_programming();
        }
    }

}
