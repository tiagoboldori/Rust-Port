use super::cliente;

pub struct Roteador{
    pub clientes: Vec<cliente::Cliente>
}


impl Roteador{
    pub fn new() -> Self{
        Self { clientes:vec![] }
    }
    
    
    pub fn add_cliente(&mut self, cli:cliente::Cliente) -> Result<(), String>{

        match self.get_cliente(cli.get_id()) {
            Some(_) => Err(String::from("Cliente já existe")),
            None => {
                self.clientes.push(cli);
                Ok(())
            }
        }
    }
    
    
    pub fn get_all_clientes(&self) -> &Vec<cliente::Cliente>{
       &self.clientes
    }


    pub fn get_cliente(&self, id:usize) -> Option<&cliente::Cliente>{
        self.clientes.get(id)
    }
}