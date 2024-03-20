use crate::console_log;



pub fn get_level(level: i32) -> Vec<Vec<i32>>{
    //0 = empty  1 = wall  2 = player starting
    let level1 = 
        vec![
            vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ]
    ;
    if level == 1{
        return level1.clone().into_iter().rev().collect::<Vec<Vec<i32>>>().to_owned();
    }
    else {
        panic!("level not defined");
    }
}