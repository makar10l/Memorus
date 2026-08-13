pub fn parse(args : &Vec<String>){
    let command = match args.get(1){
        Some(coma) => coma,
        None => return,
    };
    match command{
        "add" => add(args),
        "list" => list()
    }   
}