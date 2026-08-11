mod simulador;
use simulador::cliente;
fn main() {
    println!("Hello, world!");
    let mut cli: cliente::Cliente = cliente::Cliente::new(0);
    let mut adj0:cliente::Adj = cliente::Adj::new(1);

    cli.add_adj(adj0);
    let adj: &Vec<cliente::Adj> = cli.get_all_adj(); 
    println!("{}", adj[0].get_id());
}