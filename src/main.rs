mod simulador;
use simulador::cliente;
use simulador::roteador;


fn main() {
    println!("Hello, world!");
    
    println!("Criando Roteador");
    let mut rot: roteador::Roteador = roteador::Roteador::new();
    
    println!("Criando cliente ID:0");
    let mut cli: cliente::Cliente = cliente::Cliente::new(0);
    rot.add_cliente(cli); 

    println!("Criando struct adjacencia ->1");
    let mut adj0:cliente::Adj = cliente::Adj::new(1);

    println!("Adicionando Adjacencia em 0 para 1 | 0 -> 1");
    let mut temp_cli:Option<& mut cliente::Cliente> = rot.get_cliente(0);
    match temp_cli{
        Some(c)=> {
            c.add_adj(adj0);
            println!("Buscando adjacencias para Cliente 0");
            let adj: &Vec<cliente::Adj> = c.get_all_adj(); 
            println!("{}", adj[0].get_id());
        },
        _ => println!("Erro ao buscar cliente ID:0")
    }
    
    
}