use rayon::{self, prelude::*};
use ccl;
use ccl::dhashmap::DHashMap;

use std::collections::HashMap;

fn main() {

    // let hm = HashMap::new();

    let start = std::time::Instant::now();
    
    parallel(9, 10, 1);
    
    let end = std::time::Instant::now();

    println!{"run time: {:?}", (end -start).as_millis()}
}


fn chunkify(start: u64, end: u64, num_chunks: u64) -> Vec<Vec<u64>> {
    let div_size: u64 = (end - start) / num_chunks;

    let mut prev_i = start;

    let mut ret : Vec<Vec<u64>> = Vec::with_capacity(num_chunks as usize);


    for i in 0..num_chunks{
        let chunk_start = prev_i;

        
        let chunk_end =  
        if i == num_chunks -1 {
            end
        } else{
            prev_i + div_size
        };

        let k  =(chunk_start..chunk_end).into_iter().collect::<Vec<_>>();
        ret.push(k);

        println!{"chunk starting at {} and ending at {}",chunk_start, chunk_end}
        prev_i += div_size;
    }   

    ret
}



fn parallel(start: u64, end: u64, chunks: u64) {
    
    let mut hm_ = HashMap::new();
    let chunks = chunkify(start, end, chunks);
    
    for chunk in chunks{

        let mut hm_vec = 
        chunk.into_par_iter()
            .map(|num| {
                let mut new_hm = HashMap::new();

                run_collatz(num, &hm_, &mut new_hm);
                new_hm
            }).collect::<Vec<_>>();

            
        for i in 0..hm_vec.len() {
            let indiv_hm = hm_vec.remove(0);

            for (k, v) in indiv_hm{
                if !hm_.contains_key(&k){
                    hm_.insert(k,v);
                }
            }
        }

        dbg!{&hm_};

        // store_data(hm_);
    }

}


fn store_data(hm: HashMap<u64, u64>) {
    unimplemented!()
}


fn run_collatz(number: u64, previous_kv: &HashMap<u64, u64>, write_kv : &mut HashMap<u64, u64>) {
    let mut num = number;

    let mut vec = Vec::with_capacity(number as usize/ 4);

    vec.push(num);
    
    loop{ 
        let temp = 
        if num % 2 == 0 {
            num / 2
        } else {
            num*3 + 1
        };

        dbg!{num};

        if insert_hm(num, &temp, previous_kv, write_kv) {break}
        num = temp;

        if num == 1{
            break
        }

    }
}

fn insert_hm(prev: u64, current: &u64, ref_map: &HashMap<u64, u64>, new_map: &mut HashMap<u64, u64>) -> bool {
    if !ref_map.contains_key(&current) {
        new_map.insert(prev, *current);
        false
    } else {
        true
    }
}