struct Rectangulo {
    alto: i32,
    ancho: i32
}
//let mut nombres: Vec<String> = Vec::new();        ARRAYLIST
fn main() {
    loop{
        let w_a: Rectangulo = obtener_rectangulo();
        let bins: Rectangulo = obtener_rectangulo();
        
        if bins.alto < w_a.alto && bins.ancho < w_a.ancho {
            //CREAMOS LA MATRIZ DE ESPACIO VACIO
            let mut wa_space_array: Vec<Vec<i32>> = inicializar_space_array(&w_a);
                                 
            //CALCULAR CANTIDAD DE BINS EN EL WORK_AREA
                //Bins = TamañoWA/TamañoBins
            let amount_bins_ancho:i32= w_a.ancho / bins.ancho;
            let amount_bins_alto: i32 = w_a.alto / bins.alto;

            println!("Caben  {} Bins a lo ancho", amount_bins_ancho);
            println!("Caben  {} Bins a lo alto", amount_bins_alto);

            //ESTA ES LA CANTIDAD DE BINS QUE CABEN EN EL WORK AREA
            //let amount_bins = cmp::min(amount_bins_alto, amount_bins_ancho);
            let amount_bins = amount_bins_ancho*amount_bins_alto;
            

            //LLENAMOS EL ARREGLO DEL WORK AREA CON EL NUMERO DEL BIN PARA VIZUALIZARLOS
            let mut j_w_a: i32;
            let mut k_w_a: i32;
            
            for i in 0..amount_bins {

                for j in 0..bins.alto {
                    //j_w_a = j+(i%bins.alto*bins.alto);
                    j_w_a = j+((i%(bins.alto))*bins.alto);
                    

                    for k in 0..bins.ancho{
                        //k_w_a = k+(i%bins.ancho*bins.ancho);
                        k_w_a = k+((i%(bins.ancho)) * bins.ancho);
                        println!("I: {} = j_w_a: {} | {}, k_w_a: {} | {}", i, j_w_a, j, k_w_a, k);
                        //wa_space_array[j_w_a as usize][k_w_a as usize] = i+1;
                        //  mostrar_array(&wa_space_array, &w_a);
                    }
                }
            }
            mostrar_array(&wa_space_array, &w_a);
            //mostrar Bins En el Work Area


            //FUNCIONA - A PEDIR LOS ITEMS
            let mut _items: Vec<Rectangulo> = Vec::new();
            //loop {
                //CANTIDAD DE ITEMS
                //let items_amout: i32 = get_size("cantidad de items".to_string());
                //PEDIR ITEMS
                //println!("cantidad de items: {}", items_amout);
            //}
            break;
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

fn inicializar_space_array(w_a: &Rectangulo)->Vec<Vec<i32>>{
    let mut w_a_space: Vec<Vec<i32>> = Vec::new();
    for (i, _col) in (0..w_a.alto).enumerate() {
        for (_j, _raw) in (0..w_a.ancho).enumerate(){
            //w_a_space[i][j] = 0;
            w_a_space.push(Vec::new());
            w_a_space[i].push(0)
        }                    
    }
    w_a_space
}
fn mostrar_array(array: &Vec<Vec<i32>>, w_a: &Rectangulo){
    for (i, _col) in (0..w_a.alto).enumerate() {
        for (j, _raw) in (0..w_a.ancho).enumerate(){
            print!("[{}]", array[i][j]);
        }                    
        println!("");
    }
}
fn obtener_rectangulo() -> Rectangulo{
    let w_a = Rectangulo{
        alto: get_size("alto del espacio de trabajo".to_string()),
        ancho: get_size("ancho del espacio de trabajo".to_string())
    };
    w_a
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


