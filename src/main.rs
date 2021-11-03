use std::io;

fn main() {

    //let mut stop = String::new();

    loop {

        println!("How many rows ? : ");
        let mut rows = String::new();
        io::stdin().read_line(&mut rows).expect("Error: cannot read rows input");   

        let rows: u32 = match rows.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("How many columns ? : ");
        let mut cols = String::new();
        io::stdin().read_line(&mut cols).expect("Error: cannot read cols input");   

        let cols: u32 = match cols.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("\n\n\t\t\t\tMULTIPLICATION TABLE with {} rows and {} columns", rows, cols);

        let mut iters: f32 = cols as f32;
        iters = (iters*16.2).ceil();

        let iter: u32 = iters as u32;

        for _x in 1..iter {
            print!("-");
        }
        println!("");

        for i in 1..rows+1 {
            print!("|");
            for j in 1..cols+1 {
                if j == cols {
                    print!("\t{}\t", i*j);
                }
                else{
                    print!("\t{}\t|", i*j);
                }
            }
            println!("|");
            for _x in 1..iter {
                print!("-");
            }
            println!("");
        }

        /*println!("Again? y/n : ");
        io::stdin().read_line(&mut stop).expect("Error cannot read input!");

        println!(" WHAT : {}", stop.eq("n"));

        if stop.eq("n"){
            println!("Howya ")
            break;
        }*/
    }
}
