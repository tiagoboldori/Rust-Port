use super::cliente;

pub struct Roteador{
    pub clientes: Vec<cliente::Cliente>
}


impl Roteador{
    pub fn new() -> Self{
        Self { clientes:vec![] }
    }
}