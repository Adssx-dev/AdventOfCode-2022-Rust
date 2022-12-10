
pub fn examine_rows(height : usize,width :  usize,  forest : & Vec<Vec<i32>>, bool_forest : &mut Vec<Vec<bool>>) {
    for row in 0..height {
        let mut row_max = 0;
        for col in 0..width {
            let tree_height = forest[row][col];
            if tree_height > row_max || col == 0 || col == height - 1 {
                bool_forest[row][col] = true;
                row_max = tree_height;
            }
        } 
    }
}

pub fn examine_cols(height :  usize, width : usize, forest : & Vec<Vec<i32>>, bool_forest : &mut Vec<Vec<bool>>) {
    for col in 0..width {
        let mut col_max = 0;
        for row in 0..height {
            let tree_height = forest[row][col];
            if tree_height > col_max  || row == 0 || row == width - 1 {
                bool_forest[row][col] = true;
                col_max = tree_height;
            }
        } 
    }
}

pub fn examine_rows_rev(height : usize,width :  usize,  forest : & Vec<Vec<i32>>, bool_forest : &mut Vec<Vec<bool>>) {
    for row in 0..height {
        let mut row_max = 0;
        for col in (0..width).rev() {
            let tree_height = forest[row][col];
            if tree_height > row_max || col == 0 || col == height - 1 {
                bool_forest[row][col] = true;
                row_max = tree_height;
            }
        } 
    }
}

pub fn examine_cols_rev(height :  usize, width : usize, forest : & Vec<Vec<i32>>, bool_forest : &mut Vec<Vec<bool>>) {
    for col in 0..width {
        let mut col_max = 0;
        for row in (0..height).rev() {
            let tree_height = forest[row][col];
            if tree_height > col_max  || row == 0 || row == width - 1 {
                bool_forest[row][col] = true;
                col_max = tree_height;
            }
        } 
    }
}

pub fn day8_pt1() -> i32 {
    let file = include_str!("../../inputs/day8.txt");

    let forest : Vec<Vec<i32>> = file.lines()
        .map(|l| l.chars()
            .map(|c| c as i32 - '0' as i32).collect()
    ).collect();

    let height = forest.len();
    let width = forest.first().unwrap().len();

    let mut bool_forest : Vec<Vec<bool>> = vec![vec![false;width]; height];

    examine_rows(height, width, &forest, &mut bool_forest);
    examine_rows_rev(height, width, &forest, &mut bool_forest);
    examine_cols(height, width, &forest, &mut bool_forest);
    examine_cols_rev(height, width, &forest, &mut bool_forest);

    bool_forest.iter()
        .map(|row| row.iter()
            .map(|elem| *elem as i32)
            .sum::<i32>())
        .sum::<i32>()

}


pub fn top_view(row : usize, col : usize, forest : &Vec<Vec<i32>>) -> i32 {
    let mut view = 0;
    let mut current_row = row;
    let tree_height = forest[row][col];
    loop {
        if current_row == 0 {
            break;
        }
        current_row -= 1;
        view += 1;
        if forest[current_row][col] >= tree_height {
            break;
        }
    }
    std::cmp::max(view, 1)
}

pub fn bottom_view(row : usize, col : usize, forest : &Vec<Vec<i32>>) -> i32 {
    let mut view = 0;
    let mut current_row = row;
    let tree_height = forest[row][col];
    loop {
        if current_row == 98 {
            break;
        }
        current_row += 1;
        view += 1;
        if forest[current_row][col] >= tree_height {
            break;
        }
    }
    std::cmp::max(view, 1)
}



pub fn left_view(row : usize, col : usize, forest : &Vec<Vec<i32>>) -> i32 {
    let mut view = 0;
    let mut current_col = col;
    let tree_height = forest[row][col];
    loop {
        if current_col == 0 {
            break;
        }
        current_col -= 1;
        view += 1;
        if forest[row][current_col] >= tree_height {
            break;
        }
    }
    std::cmp::max(view, 1)
}


pub fn right_view(row : usize, col : usize, forest : &Vec<Vec<i32>>) -> i32 {
    let mut view = 0;
    let mut current_col = col;
    let tree_height = forest[row][col];
    loop {
        if current_col == 98 {
            break;
        }
        current_col += 1;
        view += 1;
        if forest[row][current_col] >= tree_height {
            break;
        }
    }
    std::cmp::max(view, 1)
}

pub fn day8_pt2() -> i32 {
    let file = include_str!("../../inputs/day8.txt");

    let forest : Vec<Vec<i32>> = file.lines()
        .map(|l| l.chars()
            .map(|c| c as i32 - '0' as i32).collect()
    ).collect();

    let height = forest.len();
    let width = forest.first().unwrap().len();

    let mut view_forest : Vec<Vec<i32>> = vec![vec![0;width]; height];

    //top_view(3, 0, &forest)

    for row in 0..height {
        for col in 0..width {
            let top = top_view(row, col, &forest);
            let bottom = bottom_view(row, col, &forest);
            let left = left_view(row, col, &forest);
            let right = right_view(row, col, &forest);
            view_forest[row][col] =  top * bottom * left * right;
        }
    }

    *view_forest.iter().flatten().max().unwrap()
    

}