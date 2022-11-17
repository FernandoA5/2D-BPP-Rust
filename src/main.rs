pub mod rectangulo;
pub mod lector_instancias;

use rand::Rng;
use rectangulo::Rectangulo as Rec;
use lector_instancias::Instancia;

use std::fs::OpenOptions;
use std::io::prelude::*;

use std::time::{Instant};
const VERBOSE:bool = true;
const ALPHA: f64 = 0.6;

//QUIZÁ SEA MEJOR IDEA AÑADIR EL ID COMO PROPIEDAD DEL ITEM. ESO PODRÍA CAMBIAR BASTANTE TODO El CÓDIGO, PERO PIENSO ES LA MEJOR FORMA PARA EVITAR TANTOS PARCHES RESPECTO A IDENTIFICAR CADA ITEM
fn main() {
    loop{
        //EMPEZAMOS CON LA HEURÍSTICA CONTRUCTIVA
        //std::process::Command::new("clear").status().unwrap();
       // let (mut bins_array, mut lista_soluciones, bins, items, num_inicial, cont_usados, wasted_space, inst) = heuristica_constructiva();

        //HAY QUE HACER LA HEURÍSTICA ALEATORIA:
        heuristica_aleatoria();
        //AQUÍ EMPEZAMOS CON LA HEURÍSTICA DE MEJORA
        //heuristica_mejora(&mut bins_array, &mut lista_soluciones, &bins, &items, num_inicial, &cont_usados, &wasted_space, &inst);
         
}
fn heuristica_aleatoria(){
    //LEEMOS LA INSTANCIA
    let (contenedores_rec, items_rec, inst): (Vec<Rec>, Vec<Rec>, Instancia) = variables_de_instancia();
    let now: Instant = Instant::now();
    //(bins.clone(), bins_array, items, lista_soluciones, num_inicial)
    let (bins, mut bins_array, items, mut lista_soluciones, num_inicial): (Vec<Rec>, Vec<Vec<Vec<char>>>, Vec<Rec>, Vec<String>, usize) = obtener_contenedores_items(&contenedores_rec, &items_rec);

    let lista_indices_items: Vec<usize> = obtener_indices_de_items(&items);
    //dmax - alfa (dmax-dmin) Aquí el mientras más cerca de 1 mas greedy, y mientras más cerca de 0 más aleatorio. Y tomamos los mayores al numero obtenido por la formula para elegir uno aleatoriamente
    let mut copia_items: Vec<Rec> = items.clone(); //COPIAMOS LA LISTA ORIGINAL PARA PODER USARLA Y ELIMINAR DE ELLA LOS ITEMS QUE SE VAYAN INCRUSTANDO
    //NECESITAMOS DEFINIR ALFA: la lista de items ya está ordenada en este punto, so:
    //AQUÍ DEBERÍA EMPEZAR EL LOOP
    let gen_sol: bool = true; //ESTO HACE QUE DESPUES DE INSERTAR EL ITEM SE GUARDE SU SOLUCIÓN EN LA LISTA DE SOLUCIONES.
    let mut items_acomodados: i32 =0;
    loop{
        //YA OBTUVIMOS LA LISTA DE INDICES DE LOS ITEMS ORDENADOS ORIGINALES, AHORA, NECESITO VINCULARLOS A LOS ITEMS QUE SE PASAN POR LA FUNCIÓN UMBRAL Y LOS QUE SALEN

        let (max_area_item, min_area_item): (&Rec, &Rec) = (&copia_items[0], &copia_items[copia_items.len()-1]); //ESTO FUNCIONA AHORA QUE LA LISTA INICIAL DE ITEMS ESTÁ INTEGRA, PERO HABRÁ QUE USAR UNA COPIA DE ESA LISTA E IR ELIMINANDO DE ELLA LOS ITEMS INSERTADOS PARA EVITAR REPETICIONES.

        //CALCULEMOS EL UMBRAL. ASUMAMOS ALFA COMO 0.6 PARA MAXIMIZAR (ESTO SERÁ MÁS GREEDY QUE RANDOM).
        let dmax: f64 = max_area_item.area as f64;
        let dmin: f64 = min_area_item.area as f64;

        let umbral: f64 = dmax - ((ALPHA) * (dmax-dmin));
        let (items_sobre_umbral, id_items_sobre_umbral): (Vec<Rec>, Vec<usize>) = obtener_items_encima_del_umbral(&copia_items, umbral, &lista_indices_items);
        //WELL, AHORA, DE ESOS ITEMS POR ENCIMA DE EL UMBRAL, ELEGIMOS UNO ALEATORIAMENE.
        //GENERAMOS ALEATORIAMENTE EL ÍNDICE DEL ITEM QUE INCRUSTAREMOS
        let indice_item_random = if id_items_sobre_umbral.len() > 1 { 
            rand::thread_rng().gen_range(0..id_items_sobre_umbral.len()-1)
        } else {0};
        
        let mut lista_un_item: Vec<Rec> = Vec::new();
        
        if items_sobre_umbral.len() > 0{
            lista_un_item.push(items_sobre_umbral[indice_item_random].clone());
        }else{
            lista_un_item.push(copia_items[items_sobre_umbral.len()].clone())
        }

        items_acomodados+= colocar_items(&lista_un_item, &bins, &mut bins_array, &mut lista_soluciones, gen_sol, indice_item_random);

        copia_items.remove(indice_item_random);
        //PODEMOS HACER DOS COSAS, INCRUSTAR EL ITEM AHORA, Y REPETIR TODO EL PROCESO MEDIANTE UN LOOP DESDE LA DEFINICIÓN DE MAX Y MIN AREA ITEMS. O NO INCRUSTAMOS EL ITEM, PERO GENERAMOS UNA LISTA DE ITEMS PARA INCRUSTAR Y LUEGO LO HACEMOS CON EL MÉTODO DE ACOMODAR LLAMADO UNA SOlA VEZ EN LUGAR DE N.
        if copia_items.len() == 0 {
            break;
        }
    };
    mostrar_contenedores_llenos(&mut bins_array, &bins);
    //ESTO TIENE UN PROBLEMA, A LA FUNCIÓN SE LE PASA UN NUMERO INICIAL DESDE EL CUAL ITERAR, SI LA LISTA ESTÁ RANDOMIZADA POR EL PROCESO DE ALFA Y EL UMBRAL, ENTONCES IGNORARÁ LOS IDs REALES DE LOS ITEMS Y LOS ENUMERARÁ DESDE EL NÚMERO INICIAL, SO: DEBEMOS USAR LA PRIMERA OPCION DE INCRUSTARLOS UNO POR UNO.
    println!("Items_insertados: {}", items_acomodados);
}
fn obtener_items_encima_del_umbral(lista_items: &Vec<Rec>, umbral: f64, lista_indices_items: &Vec<usize>) -> (Vec<Rec>, Vec<usize>){
    let mut nueva_lista_items: Vec<Rec> = Vec::new(); //INICIALIZAMOS LA LISTA DE ITEMS SOBRE EL UMBRAL
    let mut lista_id_items_umbral: Vec<usize> = Vec::new(); //INICIALIZAMOS LA LISTA DE SUS IDs
    let mut lista_id_real_items: Vec<usize> = Vec::new();
    let mut contador: i32 = 0;
    for item in lista_items{
        contador+=1; //EL CONTADOR DEFINIRÁ EL ID
        if item.area as f64 > umbral{ //SI EL AREA ESTÁ BAJO EL UMBRAL INSERTAMOS EL ITEM EN LA LISTA
            nueva_lista_items.push(item.clone());
            lista_id_items_umbral.push(contador as usize);
        }
        else{ break } //COMO ESTÁ ORDENADA LA LISTA DE ITEMS, CUANDO UNO NO CUMPLA LA CONDICIÓN, EL RESTO TAMPOCO LO HARÁ.
    }
    (nueva_lista_items, lista_id_items_umbral) //RETORNAMOS LA LISTA DE ITEMS Y SUS IDs
}
fn heuristica_constructiva() -> (Vec<Vec<Vec<char>>>, Vec<String>, Vec<Rec>, Vec<Rec>, usize, i32, i32, Instancia){
    println!("\nIMPLEMENTACIÓN 2D BPP");
        //LEEMOS LA INSTANCIA
        let (contenedores_rec, items_rec, inst):(Vec<Rec>, Vec<Rec>, Instancia) =variables_de_instancia();
        let now = Instant::now();  
        
        let (bins, mut bins_array, items, mut lista_soluciones, num_inicial): (Vec<Rec>, Vec<Vec<Vec<char>>>, Vec<Rec>, Vec<String>, usize) = obtener_contenedores_items(&contenedores_rec, &items_rec);
        //COMENZAR A ALMACENAR LOS ITEMS EN LOS CONTENEDORES
        let items_acomodados:i32=colocar_items(&items, &bins, &mut bins_array, &mut lista_soluciones, true, num_inicial);
        println!("Items insertados: {}", items_acomodados);
        //MOSTRAR LO HECHO EN PANTALLA -> PODEMOS IMPRIMIR INDIVIDUALMENTE CADA CONTENEDOR

        let (cont_usados, wasted_space) = mostrar_contenedores_llenos(&mut bins_array, &bins);

        if (items_acomodados as usize) < items.len() {
            println!("No se pudieron insertar todos los items");
        } 
        let new_now = Instant::now();
        println!("Tiempo: {:?}", new_now.duration_since(now));
        //(mut bins_array, mut lista_soluciones, bins, items, num_inicial, cont_usados, wasted_space, inst)
        (bins_array.clone(), lista_soluciones.clone(), bins.clone(), items.clone(), num_inicial, cont_usados, wasted_space, inst.clone())
}
fn heuristica_mejora(bins_array: &mut Vec<Vec<Vec<char>>>, lista_soluciones: &mut Vec<String>, bins: &Vec<Rec>, items: &Vec<Rec>, num_inicial:usize, cont_usados: &i32, wasted_space: &i32, inst: &Instancia){
    let (mut new_cont_usados, mut new_wasted_space): (i32, i32);
        let sol_inicial: Vec<String> = lista_soluciones.clone();
        let mut bins_array_inicial: Vec<Vec<Vec<char>>> = bins_array.clone();
        let improve_heuristic_now = Instant::now();  
        let mut iteracion_constructiva = 0;
        
        loop{
            println!("------------Heuristica de mejora. Iter: {}-------------------- \n", iteracion_constructiva);
            
            //OBTENEMOS UN ITEM ALEATORIAMENTE 
            let i_remp = rand::thread_rng().gen_range(0..items.len()-1);

            println!("i_remp: {}", i_remp);
            println!("Iter_conts: {}", iteracion_constructiva);
            //OBTENEMOS UN ITEM ALEATORIAMENTE
            let larger_item:Rec = items[i_remp].clone();
            //OBTENEMOS LOS COMPONENTES DE POSICION DEL ITEM DESDE LA SOLUCIÓN
            let (n_item, i_contenedor, i_contenedor_i32, fila, col) = obtener_datos_de_solucion(lista_soluciones, i_remp);
            //MOVEMOS EL PRIMER ITEM Y RESETEAMOS EL ESPACIO A 0s
            let _caracter = calcular_caracter_de_item(n_item-1);
            
            //LIMPIAMOS LOS INDICES DEL ITEM ALEATORIO, PARA QUE ESTÉN EN 0s
            print!("Cleaning:");
            verificar_e_insertar_item_individual(bins_array, fila, col, &items, n_item-1, bins[i_contenedor].clone(), &i_contenedor_i32, '0', lista_soluciones, false, num_inicial);

            //QUITAMOS LOS ÚLTIMOS N ITEMS QUE QUEPAN EN EL AREA DEL ITEM DE MAYOR TAMAÑO Y RESETEAMOS SUS ESPACIOS A CERO

            //PRIMERO HAY QUE OBTENER ESOS N ITEMS
            let mut num_nuevo_primer_item: i32 = 0;
            let items_to_move: Vec<Rec> = obtener_items_to_move(&items, &larger_item, &mut num_nuevo_primer_item); //LISTA DE ITEMS PARA MOVER (NO INCLUYE EL QUE SE MÁS GRANDE QUE SE QUITÓ)
            //LA FUNCIÓN TAMBIÉN ACTUALIZA EL NUMERO DEL ITEM DESDE DONDE SE CONTINUARÁ LA INSERICIÓN
            
            let mut indice = num_nuevo_primer_item;
            for _item in 0..items_to_move.len(){
                let (n_item, i_contenedor, i_contenedor_i32, fila, col) = obtener_datos_de_solucion(lista_soluciones, indice as usize);
                //LIMPIAMOS ESOS ITEMS A 0s
                print!("Cleaning:");
                verificar_e_insertar_item_individual(bins_array, fila, col, &items, n_item-1, bins[i_contenedor].clone(), &i_contenedor_i32, '0', lista_soluciones, false, num_inicial);

                indice +=1;
            }
            eliminar_soluciones_obsoletas(lista_soluciones, num_nuevo_primer_item as usize);
            
            //ACOMODAMOS LOS ITEMS CON EL ALGORITMO INICIAL. FUNCION: acomodar()
            //COMENZAR A ALMACENAR LOS ITEMS EN LOS CONTENEDORES
            let num_inicial = lista_soluciones.len();
            //GUARTAR EN LA LISTA DE ITEMS A INSERTAR EL LARGER ITEM:
            let items_acomodados:i32=colocar_items(&items_to_move, &bins, bins_array, lista_soluciones, true, num_inicial);

            //INSERTAMOS EL ÚLTIMO ITEM
            let mut ultimo_item: Vec<Rec> = Vec::new();
            ultimo_item.push(larger_item);
            let ultimo_item:i32 = colocar_items(&ultimo_item, bins, bins_array, lista_soluciones, true, i_remp);
            
            //IMPRIMIR NUEVAS SOLUCIONES
            println!("Items insertados: {}", items_acomodados+ultimo_item);

            //ORGANIZAMOS LA LISTA DE SOLUCIONES 
            lista_soluciones.remove(i_remp);
            ordenar_lista_soluciones(lista_soluciones);
            
            (new_cont_usados, new_wasted_space) = contar_contenedores_usados(bins_array, &bins);
            //CRITERIO DE SALIDA
                //NECESITAMOS 2 COSAS CONTENEDORES USADOS y ESPACIO DESPERDICIADO
            if new_cont_usados < *cont_usados || new_wasted_space < *wasted_space || iteracion_constructiva >= 1200{
                break;
            }
            iteracion_constructiva+=1;
        }
        //IMPRIMIMOS LOS NUEVOS RESULTADOS 
        if new_cont_usados < *cont_usados || new_wasted_space < *wasted_space{
            println!("Mejor solución encontrada:\nContenedores_usados: {}, Espacio Desperdiciado: {} vs Contenedores_usados_iniciales: {}. Espacio Desperdiciado inicial: {}", new_cont_usados, cont_usados, cont_usados, wasted_space);
            println!("Mejor Solución: ");
            for sol in lista_soluciones{
                println!("{}", sol);
                guardar_soluciones_en_fichero(&inst, iteracion_constructiva, sol);
            }
            mostrar_contenedores_llenos(bins_array, bins);
        }
        else{
            println!("No se encontró una mejor solución:\n Conenedores_usados: {}, Espacio Desperdiciado: {} vs nContenedores_usados:{}, Desperdiciado: {}", cont_usados, wasted_space, new_cont_usados, new_wasted_space);
            println!("Mejor Solución: ");
            for sol in sol_inicial{
                println!("{}", sol);
                guardar_soluciones_en_fichero(&inst, iteracion_constructiva, &sol);
            }
            mostrar_contenedores_llenos(&mut bins_array_inicial, bins);
        }
        let new_improve_heuristic_now = Instant::now(); 
        println!("Tiempo: {:?}", new_improve_heuristic_now.duration_since(improve_heuristic_now));

}   
}
fn obtener_indices_de_items(items: &Vec<Rec>)-> Vec<usize>{
    let mut lista_indices: Vec<usize> = Vec::new();
    for i in 0..items.len(){
        lista_indices.push(i);
    }
    lista_indices
}
fn obtener_contenedores_items(contenedores_rec: &Vec<Rec>, items_rec: &Vec<Rec>, )->(Vec<Rec>, Vec<Vec<Vec<char>>>, Vec<Rec>, Vec<String>, usize){
    //LOS CONTENEDORES SE OBTIENEN A PARTIR DE LA INSTANCIA
    let bins: &Vec<Rec> = contenedores_rec;
    let bins_array: Vec<Vec<Vec<char>>> = Vec::new();

    let mut items: Vec<Rec> = items_rec.clone();
    if VERBOSE == true {
        println!("Items sin ordenar:");
        imprimir_items(&items);
    }
    //ORDENAR ITEMS DE MAYOR A MENOR
    ordenar_items(&mut items);
    if VERBOSE == true{
        println!("Ordenando items...");
        imprimir_items(&items);
    }
    //CREAMOS LA LISTA DE STRINGS QUE REPRESENTAN LA SOLUCIÓN:
    let lista_soluciones: Vec<String> = Vec::new();
    let num_inicial = lista_soluciones.len();

    (bins.clone(), bins_array, items, lista_soluciones, num_inicial)
}
fn variables_de_instancia() -> (Vec<Rec>, Vec<Rec>, Instancia){
    let mut inst: Instancia = Instancia::new();
        inst.leer_instancia(); 
        //MOSTRAMOS EL NOMBRE
        println!("Instancia: {}", inst.titulo);
        println!("Items: {}", inst.cantidad_items);
        println!("Contenedores: {}", inst.cantidad_contenedores);
        //OBTENEMOS LOS ITEMS COMO TUPLAS
        let items_instancia: Vec<(i32, i32)> =  inst.obtener_items();
        let mut items_rec: Vec<Rec> = Vec::new();
        //LOS CONVERTIMOS EN RECTANGULOS
        for (alto, ancho) in items_instancia {
            let mut rec = Rec{
                alto: alto,
                ancho: ancho,
                area: 0
            };
            rec.obtener_area();
            items_rec.push(rec);
        }
        //OBTENEMOS LOS CONTENEDORES COMO TUPLAS
        let contenedores_instancia: Vec<(i32, i32)> = inst.obtener_contenedores();
        let mut contenedores_rec: Vec<Rec> = Vec::new();
        //LOS CONVERTIMOS EN RECTÁNGULOS
        for (alto, ancho) in contenedores_instancia {
            let mut rec = Rec{
                alto,
                ancho,
                area: 0
            };
            rec.obtener_area();
            contenedores_rec.push(rec);
        }
        (contenedores_rec, items_rec, inst)
}
fn guardar_soluciones_en_fichero(inst: &Instancia, iteracion_constructiva: usize, contenido: &String){
    //ABRIMOS/PREPARAMOS EL ARCHIVO DONDE GUARDAREMOS LAS SOLUCIONES
    let mut archivo = OpenOptions::new().create(true).write(true).open(format!("solutions/sol-{}.txt", inst.titulo)).unwrap();
    archivo.write_all("".as_bytes()).unwrap();
    //ABRIMOS/CREAMOS EL ARCHIVO DONDE GUARDAREMOS LA INFORMACIÓN DE INSERTADO
    let mut archivo = OpenOptions::new().append(true).create(true).open(format!("solutions/{}-sol-{}.txt", iteracion_constructiva,inst.titulo)).unwrap();
    //ESCRIBIMOS LA INFORMACIÓN DE INSERTADO EN EL ARCHIVO
    archivo.write_all(contenido.as_bytes()).unwrap();

}
fn ordenar_lista_soluciones(lista_soluciones: &mut Vec<String>){
    let mut aux: String;
    for i in 0..lista_soluciones.len(){
        for j in 0..lista_soluciones.len(){
            let sol_i_comp: Vec<&str> = lista_soluciones[i].split(",").collect();
            let sentence_n_item_i: Vec<&str> = sol_i_comp[0].split(":").collect();
            let n_item_i: usize = sentence_n_item_i[1].trim().parse::<i32>().unwrap() as usize;

            let sol_j_comp: Vec<&str> = lista_soluciones[j].split(",").collect();
            let sentence_n_item_j: Vec<&str> = sol_j_comp[0].split(":").collect();
            let n_item_j: usize = sentence_n_item_j[1].trim().parse::<i32>().unwrap() as usize;
            if n_item_i < n_item_j{
                aux = lista_soluciones[j].clone();
                lista_soluciones[j] = lista_soluciones[i].clone();
                lista_soluciones[i] = aux.clone();
            }
        }
    }
}
fn eliminar_soluciones_obsoletas(lista_soluciones: &mut Vec<String>, indice: usize){
    //println!("{}", lista_soluciones.len());
    let mut contador:i32 = 0;
    let limit = lista_soluciones.len();
    for _i in 0..limit{
        if contador == limit as i32 - indice as i32{
            
            break;
        }
        lista_soluciones.pop();
        contador+=1;
    }
}
fn mostrar_contenedores_llenos(bins_array: &mut Vec<Vec<Vec<char>>>, bins: &Vec<Rec>) -> (i32, i32){
    let (cont_usados,  wasted_space) = contar_contenedores_usados(bins_array, &bins);
        
    println!("Contenedores usados: {}", cont_usados); let mut i_bin =0;
    println!("Espacio desperdiciado: {}", wasted_space);
    if VERBOSE == true {
        for bin in bins{
            println!("Contenedor: {}", i_bin+1);
            mostrar_array(&bins_array[i_bin], &bin);
            i_bin +=1;
        }
    }
    (cont_usados, wasted_space)
}
fn obtener_datos_de_solucion(lista_soluciones: &mut Vec<String>, indice_sol: usize) -> (usize, usize, i32, i32, i32){
    let sol_larger_item: Vec<&str> = lista_soluciones[indice_sol].split(",").collect();
    let sentence_n_item: Vec<&str> = sol_larger_item[0].split(":").collect();
    let n_item: usize = sentence_n_item[1].trim().parse::<i32>().unwrap() as usize;

    let sentence_contenedor: Vec<&str> = sol_larger_item[1].split(":").collect();
    let contenedor: i32 = sentence_contenedor[1].trim().parse::<i32>().unwrap();
    let i_contenedor: usize = contenedor as usize -1;
    let i_contenedor_i32: i32 = i_contenedor as i32;

    let sentence_fila: Vec<&str> = sol_larger_item[2].split(":").collect();
    let fila: i32 = sentence_fila[1].trim().parse::<i32>().unwrap();

    let sentence_col: Vec<&str> = sol_larger_item[3].split(":").collect();
    let col:i32 = sentence_col[1].trim().parse::<i32>().unwrap();

    (n_item, i_contenedor, i_contenedor_i32, fila, col)
}
fn obtener_items_to_move(items: &Vec<Rec>, larger_item: &Rec, num_nuevo_primer_item: &mut i32)->Vec<Rec>{
    let mut items_to_move: Vec<Rec> = Vec::new(); //CREAMOS LA LISTA DE ITEMS
    let mut sum_area: i32 = 0; //CONTADOR DE AREA TOTAL
    let mut num_item:i32 = items.len() as i32;
    for item in items.iter().rev(){ //RECORREMOS AL INVERSO LA LISTA DE ITEMS
        //NECESITO DETERMINAR EL NÚMERO DE ITEM DEL ÚLTIMO DE ESTOS
        if sum_area+item.area <= larger_item.area{ //REVISAMOS QUE NO EXEDAMOS EL AREA DEL ITEM DESPLAZADO
            items_to_move.push(item.clone()); //METEMOS EL ITEM EN LA NUEVA LISTA
            sum_area+= item.area; //ACUALIZAMOS EL CONTADOR DE AREA
            num_item-=1;
        }
        else{ break; } //SI YA NO CABEN MÁS ITEMS SALIMOS DEL CICLO
    }
    *num_nuevo_primer_item=num_item;
    items_to_move.reverse();
    items_to_move //RETORNAMOS EL ITEM
}
fn verificar_e_insertar_item_individual(bins_array: &mut Vec<Vec<Vec<char>>>, fila: i32, col:i32, items: &Vec<Rec>, indice_item: usize, bin:Rec, indice_cont: &i32, caracter: char, lista_soluciones: &mut Vec<String>, gen_sol: bool, num_inicial: usize){
    let mut contador_disp:i32=0;
    let mut coor_insert: Vec<(usize, usize)> = Vec::new();

    let i = obtener_coordenada_lineal(fila as usize, col as usize, bin.clone());

    verificar_disponibilidad_espacio(items, indice_item, i, bin.clone(), bins_array, indice_cont, &mut contador_disp, &mut coor_insert, calcular_caracter_de_item(indice_item));//GENERA LOS INDICES Y VERIFICA SU DISPONIBILIDAD
    

    if coor_insert.len() == 0{
        println!("Pista: {}", calcular_caracter_de_item(indice_item));
        println!("Algo no funciona! {}", calcular_caracter_de_item(indice_item));
    }

    let mut insertado=false;
    //SI ESTAN DISPONIBLES LO INSERTAMOS
    //println!("{}", coor_insert.len());
    if contador_disp > 0 {
        insertar_item(&coor_insert, indice_item, bins_array, indice_cont, lista_soluciones, &mut insertado, caracter, gen_sol, num_inicial);
    }
    //mostrar_array(&bins_array[*indice_cont as usize], &bin);

}
fn obtener_coordenada_lineal(coor_i: usize, coor_j: usize, bin:Rec) -> usize{
    
    let (i_c, j_c) = (coor_i, coor_j); // SACA LOS COMPONENTES i y J DE EL PRIMER ESPACIO DISPONIBLE
    let n_ec: usize = (bin.ancho as usize * i_c) + j_c; //CONVIERTE EN NUMERO LINEAL LAS COORDENADAS 
    n_ec
}
fn calcular_caracter_de_item(indice: usize) -> char{
     //REINICIEMOS LOS SÍMBOLOS DISPONIBLES CUANDO SE TERMINEN
    let mut incremento = 33+(if indice >= 90{ indice-33} else {indice}); //SI LOS CARACTERES SE SALEN DE RANGO 2 VECES EL REINÍCIO YA NO SERVIRÁ
    //CAMBIAMOS EL CERO POR OTRO SÍMBOLO PARA EVITAR AMBIGUEDADES
    incremento = if incremento == 48 {33} else{incremento};
    let character:char = char::from_u32(incremento as u32).unwrap();
    character
}
fn insertar_item(coor_insert: &Vec<(usize, usize)>, indice:usize, bins_array: &mut Vec<Vec<Vec<char>>>, i_b: &i32, lista_soluciones: &mut Vec<String>, insertado: &mut bool, caracter: char, gen_sol: bool, num_inicial: usize){
    for (i_comp, j_comp) in coor_insert.clone(){
        //AQUÍ INSERTAN LOS ITEMS EN LA MATRIZ
        bins_array[*i_b as usize][i_comp][j_comp] = caracter;
    }
    //SI LO INSERTAMOS, PONEMOS UNA VARIABLE BOOL DE INSERTADO
        //GUARDAMOS lA INFORMACIÓN DE INSERTADO EN UNA CADENA DE TEXTO
    
    let contenido: String = format!("item:{}, contenedor: {}, fila:{}, col: {}\n", num_inicial+indice+1, (i_b+1), coor_insert[0].0, coor_insert[0].1);
    if VERBOSE == true{
        //MOSTRAMOS LA SOLUCIÓN DE ITEM EN PANTALLA
        print!("{}", contenido);
        //GUARDAMOS LA SOLUCIÓN DEL ITEM EN LA LISTA DE SOLUCIONES
        if gen_sol==true{
            lista_soluciones.push(contenido.clone());
        }
    }
    *insertado=true;
}
fn verificar_disponibilidad_espacio(items: &Vec<Rec>, indice: usize, i: usize, b: Rec, bins_array: &mut Vec<Vec<Vec<char>>>, i_b: &i32, contador_disp: &mut i32, coor_insert: &mut Vec<(usize, usize)>, caracter: char){
    for j in 0..(items[indice].area) as usize{// C/ITERACIÓN ES UN INDICE SIGUIENTE DEL ITEM DESDE EL INDICE i
        
        //ESTO NOS GENERA LOS INDICES QUE DEBERÍAN ESTAR DISPONIBLES PARA GUARDAR EL ITEM PARTIENDO DESDE I
        
        //A PARTIR DE UN SOLO INDICE
        let desde:f64 = i as f64; let len_f64:f64 = b.ancho as f64;
        let reinicio_ancho: usize = (desde % len_f64) as usize; //SE REINICIA A CERO CADA QUE CAMBIA DE FILA.
        let reinicio_alto: usize = (desde / len_f64).floor() as usize;
        //DEBEN SER CONTIGUOS LOS ESPACIOS DEL ANCHO
        let condicion_ancho:bool = (reinicio_ancho) as i32 + items[indice].ancho <= b.ancho;
        let condicion_alto: bool =reinicio_alto as i32 + items[indice].alto <= b.alto;

        if condicion_ancho && condicion_alto{ 
            let an_i: f64 = items[indice].ancho as f64; 
            let jump_fl:f64 = (j as f64 / an_i as f64).floor();
            let j_f64:f64 = j as f64;
            //ECUACIÓN PARA OBTENER EL INDICE DE 1DIMENSIÓN
            let ec = ((desde + (jump_fl * len_f64)) + j_f64 - (jump_fl * an_i)) as usize;

            //NICE: AHORA NECESITAMOS QUE NOS GENERE LOS ESPACIOS DISPONIBLES A PARTIR DE 2 INDICES
            //OBTENEMOS LOS 2 INDICES A PARTIR DEL NUMERO DE 1 INDICE 
            
            let i_comp:usize= (ec as f64 / len_f64).floor() as usize;    
            let j_comp:usize= ec % len_f64 as usize;
            //print!(" i_comp:{}, j_comp: {}\n", i_comp, j_comp);
            //COMPROBAMOS QUE ESAS DIRECCIONES ESTÉN DISPONIBLES (0 es disponible)
            if ec >= b.area as usize {break}
            if bins_array[*i_b as usize][i_comp][j_comp] == caracter{
                //PARA ESO NECESITAMOS UN CONTADOR
                *contador_disp+=1;
                coor_insert.push((i_comp, j_comp));
            }     
        }
        
    }
}
fn colocar_items(items: &Vec<Rec>, bins: &Vec<Rec>, bins_array: &mut Vec<Vec<Vec<char>>>, lista_soluciones: &mut Vec<String>, gen_sol: bool, num_inicial: usize)->i32{
       
    let mut acomodados: i32 =0; //CONTADOR Y DE ITEMS ACOMODADOS
    //INICIALIZAMOS LOS BINS CON 0s
    if bins_array.len() == 0{ //ASÍ EVITAMOS CREAR MAS CONTENEDORES EN LA HEURISTICA DE MEJORA
        for bin in bins{
            bins_array.push(inicializar_space_array(bin));
        }
    }
    //RECORREMOS LOS ITEMS Y ACOMODAMOS UNO POR UNO
    for i in 0..items.len() { 
        if acomodar(bins.clone(), bins_array, items, i, lista_soluciones, gen_sol, num_inicial) == true{
            acomodados+=1;
        }
    }
    acomodados
}
fn acomodar(bins: Vec<Rec>, bins_array: &mut Vec<Vec<Vec<char>>>, items: &Vec<Rec>, indice: usize, lista_soluciones: &mut Vec<String>, gen_sol: bool, num_inicial:usize) -> bool{
    let mut insertado: bool = false;
    let mut i_b: i32 = -1; //Indice Del Contenedor, Empieza en -1 POR ALGUNA RAZÓN
    for b in bins{//RECORREMOS LOS CONTENEDORES B ES EL CONTENEDOR
        i_b += 1; //SE ACTUALIZA al SIGUENTE INDICE CONTENEDOR
        if insertado == true {  //SI YA FUE INSERTADO, NOS SALIMOS DE LOS CONTENEDORES
            break;
        }
        let mut disp : Vec<(usize, usize)> = Vec::new(); //ARREGLO PARA ALMACENAR LOS INDICES DISPONIBLES
        let mut contador:i32=0; //CONTADOR PARA SABER EL AREA DISPONIBLE
        for i in 0..b.alto as usize{ //FILAS
            for j in 0..b.ancho as usize{ //COLUMNAS
                //CONTAMOS ESPACIOS DISPONIBLES
                if  bins_array[i_b as usize][i][j] == char::from_u32(48 as u32).unwrap(){ //COMPARAMOS QUE EL ARRAY CONTENGA UN 0 (0= DISPONIBLE)
                    contador+=1; //ACTUALIZAMOS EL CONTADOR
                    disp.push((i, j)); //GUARDAMOS LOS SUBINDICES DEL ESPACIO
                }
            }
        }
        if contador < items[indice].area{
            //IF EL AREA DISPONIBLE ES MENOR QUE EL AREA DEL ITEM ACTUAL PASAMOS AL SIGUIENTE CONTENEDOR
            continue;
        }
        //REVISAR QUE LOS ESPACIOS VACIÓS SEAN USABLES POR EL ITEM

        let n_ec: usize = obtener_coordenada_lineal(disp[0].0, disp[0].1, b.clone());

        //RECORREMOS LOS ESPACIOS DISPONBIES, MENOS LOS ÚLTIMOS (AREA DEL RECTANGULO) PORQUE COMPARAMOS CADA I CON SUS (AREA DEL RECTANGULO) SIGUIENTES
        for i in n_ec..(b.area as usize - (items[indice].area as usize ) ){ //RECORREMOS LOS ESPACIOS DISPONBIES. CADA ITERACIÓN ES UN POSIBLE LUGAR DONDE PONER EL ITEM
            if insertado == true{ //SI YA FUE INSERTADO NOS SALIMOS DE LOS ESPACIOS DISPONIBLES
                break;
            }

            let mut contador_disp:i32=0;
            let mut coor_insert: Vec<(usize, usize)> = Vec::new();

            //COMPARAMOS CADA I CON (SUS AREA DEL RECTANGULO) SIGUIENTES
            verificar_disponibilidad_espacio(items, indice, i, b.clone(), bins_array, &i_b, &mut contador_disp, &mut coor_insert, '0');//GENERA LOS INDICES Y VERIFICA SU DISPONIBILIDAD

            //SI ESTAN DISPONIBLES LO INSERTAMOS
            if contador_disp >= items[indice].area {
                //INSERTAR
                let caracter = calcular_caracter_de_item(num_inicial+indice);
                insertar_item(&mut coor_insert, indice, bins_array, &i_b, lista_soluciones, &mut insertado, caracter, gen_sol, num_inicial);
            }
            else{
                //println!("No se insertó");
            }
            //SI NO, LO DEJAMOS SEGUIR A LA SIGUIENTE POSIBLE POSICIÓN
            
        }//END DEL FOR QUE RECORRE LOS ESPACIOS DISPONIBLES
        //SI NO SE PUDO INSERTAR EN NINGUNA POSICIÓN, PASAMOS AL SIGUIENTE CONTENEDOR
    }//END DEL FOR QUE RECORRE LOS CONTENEDORES
    return insertado;
}
fn contar_contenedores_usados(bins_array: &mut Vec<Vec<Vec<char>>>, bins: &Vec<Rec>) -> (i32, i32){
    let mut cont_usados: i32 = 0;
    let mut wasted_space: i32 =0;
        for i in 0..bins_array.len(){
        let mut usado: bool = false;
        for j in 0..bins[i].alto as usize{
            if usado == true {break}
            for k in 0..bins[i].ancho as usize{
                if usado == true {break}
                usado = if bins_array[i][j][k] != char::from_u32(48 as u32).unwrap()
                { true } else {false}
            }
            
        }
        cont_usados += if usado == true {1} else {0}
    }
    for i in 0..cont_usados as usize{
        for j in 0..bins[i].alto as usize{
            for k in 0..bins[i].ancho as usize{
                if bins_array[i][j][k] == char::from_u32(48 as u32).unwrap(){
                    wasted_space += 1;
                }
            }
        }
    }
    (cont_usados, wasted_space)
}
fn inicializar_space_array(w_a: &Rec)->Vec<Vec<char>>{
    let mut w_a_space: Vec<Vec<char>> = Vec::new();
    for i in 0..w_a.alto as usize{
        w_a_space.push(Vec::new());
        for _j in 0..w_a.ancho{
            //w_a_space[i][j] = 0;
            w_a_space[i].push(char::from_u32(48).unwrap())
        }                    
    }
    w_a_space
}
fn mostrar_array(array: &Vec<Vec<char>>, w_a: &Rec){
    for (i, _col) in (0..w_a.alto).enumerate() {
        for (j, _raw) in (0..w_a.ancho).enumerate(){
            print!("[{}]",array[i][j]);
        }                    
        println!("");
    }
}
fn ordenar_items(items: &mut Vec<Rec>){
    let mut aux:Rec;
    for i in 0..items.len(){
        for j in 0..items.len(){
            if items[i].area > items[j].area{
                aux = items[j].clone();
                items[j]=items[i].clone();
                items[i]=aux.clone();
            }
        }
    }
}
fn imprimir_items(items: &Vec<Rec>){
    for i in 0..items.len(){
        println!("Item: {}, H: {}, W: {}, A:{}", (i+1), items[i].alto, items[i].ancho, items[i].area);
    }
}
