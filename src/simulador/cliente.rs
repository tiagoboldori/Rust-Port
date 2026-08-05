
struct Adj{
    id: usize,
    info:Info
}

struct Info{
    perda:f64,
    latencia:u32,
}

struct Cliente{
    id: usize,
    nome:String,
    adj:Vec<Adj>, 
}


impl Cliente{
    fn new(_id:usize, _nome:String) -> Self{
        let empty_v:Vec<Adj> = vec![];

        Self {
            id:_id,
            nome: _nome,
            adj: empty_v
        }
    }
    
    fn add_adj(&mut self, _adj: Adj){
        self.adj.push(_adj);
    }


    fn get_all_adj(&self) -> &Vec<Adj>{
        &self.adj
    }
}
