fn main() {
    let mut grid:Vec<Vec<i32>> = Vec::new();
    let mut row:Vec<i32> = Vec::new();
    row.push(0);
    row.push(0);
    row.push(6);
    row.push(0);
    println!("{:?}", row);
    grid.push(row);
   

    let mut row:Vec<i32> = Vec::new();
    row.push(1);
    row.push(0);
    row.push(7);
    row.push(0);
    println!("{:?}", row);
    grid.push(row);
    
    let mut row:Vec<i32> = Vec::new();
    row.push(1);
    row.push(10);
    row.push(11);
    row.push(0);
    println!("{:?}", row);
    grid.push(row);
    // println!("Grid: {:?}", grid);
    println!("*");
    println!("*");
    println!("*");
    let _resultado = devolver_resultado(grid);

    //println!("Resultado: {}", resultado);
}

fn devolver_resultado(grid:Vec<Vec<i32>>) -> i32 {
    
    let mut resultado:i32 = 0;

    
    // Suma todo el oro sin pasar por los 0
    for mut i in 0..grid.len() {
        for mut j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                println!("");
                println!("***Encontrado 0 en casilla: {}, {}***", i, j);
                i = i + 1;
                println!("**********pre-j**********: {}", j);
                if j > 0 {
                    j = j - 1;
                }
                
                println!("**********post-j**********: {}", j);
                println!("");
                break;
            } else {
                println!("{} monedas encontradas en {}, {}:", grid[i][j], i, j);
                resultado = resultado + grid[i][j];
            }
            // println!("i: {}", i);
            // println!("j: {}", j);


            println!("Suma monedas: {}", resultado);
        }
    }

    println!("Resultado funcion: {}", resultado);
    return resultado;
}
