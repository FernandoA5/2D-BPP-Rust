struct WorkArea {
    alto: i32,
    ancho: i32
}
struct Bin{
    alto: i32,
    ancho: i32
}
//let mut nombres: Vec<String> = Vec::new();        ARRAYLIST
fn main() {
    loop{
        let w_a: WorkArea = obtener_work_area();
        let bins: Bin = obtener_bin_size();
        
        if bins.alto < w_a.alto && bins.ancho < w_a.ancho {
            //FUNCIONA - A PEDIR LOS ITEMS
            let mut items: Vec<Bin> = Vec::new();

        }
        else {
            println!("Los contenedores no pueden ser más grandes que el area de trabajo");
        }
        println!("Work_Area: {}, {}", w_a.alto, w_a.ancho);
        println!("Bins size: {}, {}", bins.alto, bins.ancho);
    }
    //OBTENEMOS EL TAMAÑO DE EL AREA DE TRABAJO
    
    //OBTENEMOS EL TAMAÑO DE LOS BINS
    
    //OBTENEMOS LOS ITEMS
    
    //IMPRESIONES
}


fn obtener_work_area() -> WorkArea{
    let w_a = WorkArea{
        alto: get_size("alto del espacio de trabajo".to_string()),
        ancho: get_size("ancho del espacio de trabajo".to_string())
    };
    w_a
}
fn obtener_bin_size() -> Bin {
    let bins = Bin{
        alto: get_size("alto del contenedro".to_string()),
        ancho: get_size("ancho del contenedor".to_string())
    };
    bins
}

fn get_size(dato: String) -> i32{
    let stdin = std::io::stdin();
    loop{
        let mut dato_ingresado = String::new();
        println!("Ingresa tu {}", dato);
        stdin.read_line(&mut dato_ingresado).unwrap();
        dato_ingresado= dato_ingresado.trim().to_string();

        if dato_ingresado.len() > 0
        {
            let i_dato_ingresado: i32 = match dato_ingresado.trim().parse(){
                Ok(num) => num, 
                Err(_)=> continue
            };
            if  i_dato_ingresado > 0
            {
                break i_dato_ingresado
            }

        }

    }

}


