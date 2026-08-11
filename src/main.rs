mod simulador;
use simulador::cliente;
fn main() {
    println!("Hello, world!");
    
    println!("Criando cliente ID:0");
    let mut cli: cliente::Cliente = cliente::Cliente::new(0);
    
    println!("Criando struct adjacencia ->1");
    let mut adj0:cliente::Adj = cliente::Adj::new(1);

    println!("Adicionando Adjacencia em 0 para 1 | 0 -> 1");
    cli.add_adj(adj0);
    
    println!("Buscando adjacencias para Cliente 0");
    let adj: &Vec<cliente::Adj> = cli.get_all_adj(); 
    println!("{}", adj[0].get_id());
}