#[derive(Clone)]
pub struct Rectangulo {
    pub alto: i32,
    pub ancho: i32,
    pub area: i32
}
impl Rectangulo{
    pub fn obtener_area(&mut self){
        self.area = self.alto * self.ancho;
    }
}