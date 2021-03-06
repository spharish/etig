
use std::sync::{Arc, Mutex}; 
use std::thread; 
use std::time::Duration; 
use std::io; 
use std::fs::File; 
use std::io::{BufRead,BufReader,Read,stdin}; 
use std::env; 
 
static mut dist1_062:[[i64; 100]; 100] = [[0; 100];100]; 
static mut v_062:[[i64; 100]; 100] = [[0; 100];100]; 
static mut tmp_062:[i64;1000]=[0;1000]; 
static mut n_062:i64 = 0; 
fn input_062() -> i64 { 
    let mut input_text = String::new(); 
    io::stdin() 
        .read_line(&mut input_text) 
        .expect("failed to read from stdin"); 
 
    let trimmed = input_text.trim(); 
    match trimmed.parse::<i64>() { 
        Ok(i) => return i, 
        Err(..) =>return 0, 
    }; 
    return 0 
} 
fn get_node_062<R:Read>(reader:R)->i64 { 
    let mut reader = BufReader::new(reader).lines(); 
    let mut nn:i64=0; 
    while let Some(Ok(line)) = reader.next() { 
        let node_info= line.to_owned(); 
        nn = node_info.parse::<i64>().unwrap(); 
        break; 
    } 
    return nn; 
} 
 
fn get_graph_062<R:Read>(reader:R)->i64 { 
    let mut reader = BufReader::new(reader).lines(); 
    let mut nn:i64=0; 
    let mut k:i64=0; 
    let mut row: Vec<i64>; 
    let mut i:usize=0; 
    while let Some(Ok(line)) = reader.next() { 
        let node_info= line.to_owned(); 
        let nodes: Vec<&str> = node_info.split_whitespace().collect(); 
        if nodes.len()!=1 { 
            for j in 0..nodes.len() { 
                let mut q:i64=nodes[j].parse::<i64>().unwrap(); 
                unsafe{v_062[i as usize][j as usize]=q;} 
            } 
            i+=1; 
        } 
    } 
    return k; 
} 
 
pub fn kuch() {
	println!("dsgf");
}
fn topologicalsort_062(mut used: &mut Vec<i64>,mut ans : &mut Vec<i64>,i: i64) -> () { 
    unsafe { 
    used[i as usize]=1; 
    for k in 0..n_062 { 
        if v_062[i as usize][k as usize]!=0&&used[k as usize]==0 { 
            topologicalsort_062(&mut used,&mut ans,k); 
        } 
    } 
//    println!("here {}",i); 
    ans.push(i); 
    } 
} 
 
pub fn solve_062() -> () { 
        unsafe { 
    let file = File::open("outlav.txt").expect("Error"); 
    n_062=get_node_062(file); 
    let file2 = File::open("outlav.txt").expect("Error"); 
    let mut k:i64; 
    k=get_graph_062(file2);  
    let mut used:Vec<i64> = Vec::new(); 
    let mut ans:Vec<i64> = Vec::new(); 
    /* 
    for i in 0..n_062 { 
        for j in 0..n_062 { 
            print!("{} ", v_062[i as usize][j as usize]); 
        } 
        println!(""); 
    } 
    */ 
    for i in 0..n_062 { 
        used.push(0); 
    } 
    for i in 0..n_062 { 
        if used[i as usize]==0 { 
            topologicalsort_062(&mut used,&mut ans,i); 
        } 
    } 
    let x = ans.len(); 
    ans.reverse(); 
//    println!("here x = {}",x); 
//    println!("topological order"); 
    for i in 0..x { 
        tmp_062[i as usize]=ans[i as usize]; 
//        println!("{}",ans[i as usize]); 
    } 
//    println!("----------------"); 
    for i in 0..n_062 { 
        for j in 0..n_062 { 
            dist1_062[i as usize][j as usize]=9999999999; 
        } 
    }
    let t1 = thread::spawn( move || { 
    for s in 0..n_062/3 { 
        dist1_062[s as usize][s as usize]=0; 
            for i in 0..x { 
                let mut ll:i64 = tmp_062[i as usize]; 
                if dist1_062[s as usize][ll as usize]!=9999999999 { 
                    for k in 0..n_062 { 
                        if v_062[ll as usize][k as usize]!=0 { 
                            if dist1_062[s as usize][k as usize]>dist1_062[s as usize][ll as usize]+v_062[ll as usize][k as usize] { 
                                dist1_062[s as usize][k as usize]=dist1_062[s as usize][ll as usize]+v_062[ll as usize][k as usize]; 
                            } 
                        } 
                    } 
                } 
            } 
         
    } 
    }); 
    let t2 =thread::spawn( move || { 
    for s in n_062/3..2*n_062/3 { 
        dist1_062[s as usize][s as usize]=0; 
            for i in 0..x { 
                let mut ll:i64 = tmp_062[i as usize]; 
                if dist1_062[s as usize][ll as usize]!=9999999999 { 
                    for k in 0..n_062 { 
                        if v_062[ll as usize][k as usize]!=0 { 
                            if dist1_062[s as usize][k as usize]>dist1_062[s as usize][ll as usize]+v_062[ll as usize][k as usize] { 
                                dist1_062[s as usize][k as usize]=dist1_062[s as usize][ll as usize]+v_062[ll as usize][k as usize]; 
                            } 
                        } 
                    } 
                } 
            } 
    } 
    });
    let t3 =thread::spawn( move || { 
    for s in 2*n_062/3..n_062 { 
        dist1_062[s as usize][s as usize]=0; 
            for i in 0..x { 
                let mut ll:i64 = tmp_062[i as usize]; 
                if dist1_062[s as usize][ll as usize]!=9999999999 { 
                    for k in 0..n_062 { 
                        if v_062[ll as usize][k as usize]!=0 { 
                            if dist1_062[s as usize][k as usize]>dist1_062[s as usize][ll as usize]+v_062[ll as usize][k as usize] { 
                                dist1_062[s as usize][k as usize]=dist1_062[s as usize][ll as usize]+v_062[ll as usize][k as usize]; 
                            } 
                        } 
                    } 
                } 
            } 
    } 
    }); 
    let res1 = t1.join(); 
    let res2 = t2.join();
    let res3 = t3.join();
    for i in 0..n_062 { 
        for j in 0..n_062 { 
            if(dist1_062[i as usize][j as usize]==9999999999) { 
                print!("INF "); 
            } 
            else { 
                print!("{} ",dist1_062[i as usize][j as usize]); 
            } 
        } 
        println!(""); 
    } 
   } 
} 
