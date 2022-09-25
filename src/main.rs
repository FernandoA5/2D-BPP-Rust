
struct Rectangulo {
    alto: i32,
    ancho: i32,
    area: i32
}
impl Rectangulo{
    fn obtener_area(&self)->i32{
        self.alto * self.ancho
    }
}
//let mut nombres: Vec<String> = Vec::new();        ARRAYLIST
fn main() {
    loop{
        //std::process::Command::new("clear").status().unwrap();
        println!("IMPLEMENTACIÓN 2D BPP");
        let w_a: Rectangulo = obtener_rectangulo("area de trabajo:".to_string());
        let bins: Rectangulo = obtener_rectangulo("area de trabajo:".to_string());

        //let w_a:Rectangulo= Rectangulo{ alto: 15, ancho: 10};
        //let bins:Rectangulo= Rectangulo {alto: 6, ancho: 5};
        
        if bins.alto < w_a.alto && bins.ancho < w_a.ancho {
            //CREAMOS LA MATRIZ DE ESPACIO VACIO
            let mut wa_space_array: Vec<Vec<i32>> = inicializar_space_array(&w_a);
                                 
            //CALCULAR CANTIDAD DE BINS EN EL WORK_AREA
                //Bins = TamañoWA/TamañoBins
            let amount_bins_ancho:i32= w_a.ancho / bins.ancho;
            let amount_bins_alto: i32 = w_a.alto / bins.alto;

            println!("Caben  {} Bins a lo ancho", amount_bins_ancho);
            println!("Caben  {} Bins a lo alto", amount_bins_alto);

            //LLENAMOS EL ARREGLO DEL WORK AREA CON EL NUMERO DEL BIN PARA VIZUALIZARLOS
            llenar_arreglo_con_bins(&mut wa_space_array, &bins, amount_bins_alto, amount_bins_ancho);
            
            //MOSTRAR BINS EN EL WORK AREA
            mostrar_array(&wa_space_array, &w_a);

            //FUNCIONA - A PEDIR LOS ITEMS
            let mut items: Vec<Rectangulo> = Vec::new();
                    
            //CANTIDAD DE ITEMS
            let items_amout: i32 = get_size("cantidad de items".to_string());
            //SI EL USUARIO INGRESÓ UNA CANTIDAD VALIDA DE ITEMS
            if items_amout > 0 {
                let mut i =0;
                //PEDIMOS LOS ITEMS
                while i < items_amout {
                    let rec: Rectangulo = obtener_rectangulo(( "rectangulo ".to_string()+&(i+1).to_string()+":" ).to_string());
                    //SI EL ITEM CABE EN EL BIN SE AGREGA
                    if rec.alto <= bins.alto && rec.ancho <= bins.ancho {
                        items.push(rec);
                        i+=1;
                    }
                    //SI NO, SE PIDE DE NUEVO
                    else{
                        println!("El alto y ancho del item debe ser menor que el de el contenedor");
                    }
                }  
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
fn llenar_arreglo_con_bins(wa_space_array: &mut Vec<Vec<i32>>, bins: &Rectangulo, amount_bins_alto:i32, amount_bins_ancho: i32){
    let mut contador:i32=0;
            for i in 0..amount_bins_alto {
                for j in 0..amount_bins_ancho { 

                    contador+=1;
                    for k in 0..bins.alto {
                        let k_i:i32 = k+(bins.alto*i);
                        for l in 0..bins.ancho { 
                            let l_i:i32 = l+(bins.ancho*j);
                            
                            wa_space_array[k_i as usize][l_i as usize] = contador;
        
                        }              
                    }       

                }              
            }
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
fn obtener_rectangulo(dato: String) -> Rectangulo{
    let w_a = Rectangulo{
        alto: get_size(("alto del ".to_string()+&dato).to_string()),
        ancho: get_size(("ancho del ".to_string()+&dato).to_string()),
        area: 0
    };
    w_a.obtener_area();
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
/*fn ordenar_items(items: Vec<Rectangulo>)->Vec<Rectangulo>{
    let mut items_ordenados: Vec<&Rectangulo>;
    
    for i in 0..items.len()
    {
        items_ordenados.push(&items[i])
    }


    items_ordenados
}*/

