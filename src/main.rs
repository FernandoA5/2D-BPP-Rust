struct WorkArea {
    alto: i32,
    ancho: i32
}
struct Bin{
    alto: i32,
    ancho: i32
}
struct Item{
    _alto: i32,
    _ancho: i32
}
//let mut nombres: Vec<String> = Vec::new();        ARRAYLIST
fn main() {
    loop{

        let w_a: WorkArea = obtener_work_area();
        let bins: Bin = obtener_bin_size();
        
        if bins.alto < w_a.alto && bins.ancho < w_a.ancho {
            //CREAMOS LA MATRIZ DE ESPACIO
            let mut w_a_space: Vec<Vec<i32>> = Vec::new();

            //INICIALIZAR ESPACIO VACIO
            for (i, _col) in (0..w_a.alto).enumerate() {
                for (j, _raw) in (0..w_a.ancho).enumerate(){
                    //w_a_space[i][j] = 0;
                    w_a_space.push(Vec::new());
                    w_a_space[i].push(0)
                }                    
            }
            //MOSTRAMOS LA MATRIZ EN PANTALLA
            for (i, _col) in (0..w_a.alto).enumerate() {
                for (j, _raw) in (0..w_a.ancho).enumerate(){
                    print!("[{}]", w_a_space[i][j]);
                }                    
                println!("");
            }
            //CALCULAR CANTIDAD DE BINS EN EL WORK_AREA

            //FUNCIONA - A PEDIR LOS ITEMS
            let mut _items: Vec<Item> = Vec::new();
            loop {
                //CANTIDAD DE ITEMS
                let items_amout: i32 = get_size("cantidad de items".to_string());
                //PEDIR ITEMS
                


                println!("cantidad de items: {}", items_amout);
            }

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
fn _obtener_item() -> Item {
    let item = Item{
        _alto: 5,
        _ancho: 5
    };
    item
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


