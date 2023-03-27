use std::io;

// todo: code bot using this algorithm https://www.neverstopbuilding.com/blog/minimax

fn main() {
    let mut grid: [[u32; 3]; 3] = [[0;3];3];

    println!("Jeu de morpion");
    println!("Indiquez votre choix de cette façon: xy");

    print_grid(&grid);

    let mut player = 1;
    loop {
        if player > 2 {
            player = 1;
        }

        println!("Player {}'s turn: ", player);

        loop {
            let choice = get_input();
            
            if grid[choice.0][choice.1] == 0 {
                grid[choice.0][choice.1] = player;
                break;
            }

            println!("Case déjà prise, recommencez");
        }
                
        print_grid(&grid);

        if check_grid(&grid) {
            break;
        }

        if check_equality(&grid) {
            player = 0;
            break;
        }

        player += 1;
    }
    
    if player != 0 {
        println!("Player {} won! :D", player);
    }
    else {
        println!("It's a tie! No player won :'(")
    }
}

fn check_grid(grid: &[[u32; 3]; 3]) -> bool {
    let couples = [
        [[0,0], [0,1], [0,2]],
        [[1,0], [1,1], [1,2]],
        [[2,0], [2,1], [2,2]],
        [[0,0], [1,0], [2,0]],
        [[0,1], [1,1], [2,1]],
        [[0,0], [1,1], [2,2]],
        [[2,0], [1,1], [0,2]]
    ];

    let mut w_found: bool = false;
    let mut i: usize = 0;
    while !w_found && i < 7 {
        w_found = grid[couples[i][0][0]][couples[i][0][1]] != 0 
                    && grid[couples[i][0][0]][couples[i][0][1]] == grid[couples[i][1][0]][couples[i][1][1]]
                    && grid[couples[i][1][0]][couples[i][1][1]] == grid[couples[i][2][0]][couples[i][2][1]];
        i += 1;
    }
    return w_found
}

fn check_equality(grid: &[[u32; 3]; 3]) -> bool {
    let mut no_zero_found = true;
    let mut x = 0;
    let mut y: usize;
    while no_zero_found && x < 3 {
        y = 0;
        while no_zero_found && y < 3 {
            if grid[x][y] == 0 {
                no_zero_found = false;
            }
            y += 1;
        }
        x += 1;
    }

    return no_zero_found
}

fn print_grid(grid: &[[u32; 3]; 3]) {
    println!("  0  1  2");
    let mut i = 0;
    for line in grid {
        println!("{}{:?}", i, line);
        i += 1;
    }
}

fn get_input() -> (usize, usize) {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let y = input.chars().nth(0).unwrap().to_string();
    let x = input.chars().nth(1).unwrap().to_string();
    
    let y = usize::from_str_radix(&y, 10).unwrap();
    let x = usize::from_str_radix(&x, 10).unwrap();

    return (x, y)
}
