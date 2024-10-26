use std::f32::consts::PI;

struct GenPi{
    diameter: f64,
    lenght: f64,

}
impl  GenPi{


    fn gen_pi(&mut self){
        self.genlenght();
        self.gendiameter();

        println!("{}", self.lenght / self.diameter);

    }

    fn genlenght(&mut self){
        self.lenght = PI as f64 * self.diameter;
    }
    fn gendiameter(&mut self){
        self.diameter += 5.0;
    }
}


fn theard(){
    let mut init_pi = GenPi{diameter:4.0, lenght:0.0};
    loop {
        init_pi.gen_pi();
    }


}

fn main() {
    theard()
}

