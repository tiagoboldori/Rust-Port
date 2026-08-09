
pub struct Adj{
    pub id: usize,
    info:Info
}

impl Adj{
    pub fn new(_id:usize) -> Self{
        Self{
            id:_id,
            info: Info { perda: 0.00, latencia: 0 }
        }
    }
}




struct Info{
    perda:f64,
    latencia:u32,
}

pub struct Cliente{
    id: usize,
    adj:Vec<Adj>,
}


impl Cliente{
    pub fn new(_id:usize ) -> Self{
        let empty_v:Vec<Adj> = vec![];

        Self {
            id:_id,
            adj: empty_v
        }
    }

    pub fn add_adj(&mut self, _adj: Adj){
        self.adj.push(_adj);
    }


    pub fn get_all_adj(&self) -> &Vec<Adj>{
        &self.adj
    }
}
