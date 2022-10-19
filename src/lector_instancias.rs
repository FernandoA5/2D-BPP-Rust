use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Instancia{
    pub archivo: String,
    pub contenido: String,
    pub titulo: String,
    pub cantidad_items: i32
}
impl Instancia{
    pub fn leer_instancia(&mut self){
        let archivo: File = match File::open(&self.archivo){
            Ok(file) => file,
            Err(err) => {
                println!("Error: {}", err);
                std::process::exit(1);
            }
        };
        println!("{:?}", archivo);
        let mut buf_reader = BufReader::new(archivo);
        let mut contenido: String = String::new();
        buf_reader.read_to_string(&mut contenido).unwrap();

        self.contenido = contenido;
        self.obtener_nombre_instancia();
        self.obtener_cantidad_items();
    }
    pub fn new() -> Instancia{
        let stdin = std::io::stdin();
        let archivo: String = loop{
            let mut nombre_ingresado: String = String::new();
            println!("Ingresa el nombre del archivo:");
            stdin.read_line(&mut nombre_ingresado).unwrap();
            if nombre_ingresado.len() > 0{
                break nombre_ingresado.trim().to_string();
            }
        };
        Instancia { archivo: archivo, contenido: String::new(), titulo: String::new(), cantidad_items: 0}

    }
    pub fn obtener_nombre_instancia(&mut self){
        let contenido = self.contenido.clone();
        //LEEMOS LA PRIMER LINEA
        let lineas = contenido.lines();
        let mut title: String = String::new();
        for i in lineas { //LA LINEA 1 ES EL TITULO
            let head: Vec<&str> = i.split(":").collect();
            if head[0] == "name".to_string(){
                title=head[1].to_string();
                break;
            }
        }
        self.titulo=title;
    }
    pub fn obtener_cantidad_items(&mut self){
        let mut cantidad_items = 0;
        let contenido: String = self.contenido.clone();
        let lineas = contenido.lines();
        for linea in lineas { //LA SEGNDA LINEA ES EL ITEM
            let linea: Vec<&str> = linea.split(":").collect();
            if linea[0] == "items".to_string(){
                cantidad_items = linea[1].trim().parse::<i32>().unwrap();
                break;
            }
        }
        self.cantidad_items =cantidad_items;
    }
    pub fn obtener_items(&mut self)->Vec<(i32, i32)>{
        let mut items: Vec<(i32, i32)> = Vec::new();
        let mut item: (i32, i32) = (0, 0); //ALTO, ANCHO
        let mut contador_items = 0;
        let contenido: String = self.contenido.clone();
        let lineas = contenido.lines();
        let mut guardar = false;
        for linea in lineas { //HAY QUE ENCONTRAR DONDE EMPIEZAN LOS ITEMS
            if guardar{
                if contador_items == 100 {
                    break;
                }
                if linea == String::from(""){
                    continue;
                }
                let item_local: Vec<&str> = linea.split(",").collect();
                item.0 = item_local[0].trim().parse::<i32>().unwrap();
                item.1 = item_local[1].trim().parse::<i32>().unwrap();
                //UNA VEZ QUE EL CONTADOR LLEGUE A LA CANTIDAD DE ITEMS, SALIMOS
                items.push((item.0, item.1));
                //AL FINAL DEL GUARDADO SUMAMOS 1 AL CONTADOR
                contador_items+=1;
            }else{
                let tag: Vec<&str> = linea.split(":").collect();
                if tag[0] == String::from("items"){
                    guardar = true;
                }
            }
        }
        println!("{:?}", items);
        items
    }
    
}