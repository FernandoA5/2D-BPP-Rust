pub mod rectangulo;
pub mod lector_instancias;

use rectangulo::Rectangulo as Rec;
use lector_instancias::Instancia;

use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    loop{
        //std::process::Command::new("clear").status().unwrap();
        println!("\nIMPLEMENTACIÓN 2D BPP");
        //LEEMOS LA INSTANCIA
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
            //HAY QUE REVISAR SI LOS CONTENEDORES EXISTEN COMO ARREGLO DE CONTENEDROES
                //EN CASO DE QUE NO: HACER MODIFICACIÓN PERTINENTE

        
        //ESTO ES PARA PAUSARLO DURANTE LOS TEST
        //let _test = get_size("Test".to_string());

        //EN LAS INSTANCIAS NO SE USA AREA DE TRABAJO
        //let w_a: Rec = obtener_rectangulo("area de trabajo:".to_string());
        let mut w_a_sizes: (i32, i32) = (0, 0);
        for contenedor in &contenedores_rec {
            w_a_sizes.0 += contenedor.alto as i32;
            w_a_sizes.1 += contenedor.ancho as i32;
        };
        //let w_a: Rec = Rec { alto:w_a_sizes.0, ancho: w_a_sizes.1, area: w_a_sizes.0 * w_a_sizes.1 };
        //LOS CONTENEDORES SE OBTIENEN A PARTIR DE LA INSTANCIA
        //let bins: Rec = obtener_rectangulo("contenedor:".to_string());
        let bins: &Vec<Rec> = &contenedores_rec;
        //DATOS POR DEFECTO PARA LAS PRUEBAS
        //let w_a:Rec= Rec{ alto: 15, ancho: 10, area: 150};
        //let bins:Rec= Rec {alto: 6, ancho: 5, area: 30};
        
        // if  w_a.alto >= bins.alto &&  w_a.ancho >= bins.ancho {
            //CREAMOS LA MATRIZ DE ESPACIO VACIO
            // let mut wa_space_array: Vec<Vec<char>> = inicializar_space_array(&w_a);
                                 
            //CALCULAR CANTIDAD DE BINS EN EL WORK_AREA
            // let amount_bins_ancho:i32= w_a.ancho / bins.ancho;
            // let amount_bins_alto: i32 = w_a.alto / bins.alto;
            // let cant_bins = amount_bins_alto*amount_bins_ancho;
            // let cant_bins:i32 = inst.cantidad_contenedores;

            let mut bins_array: Vec<Vec<Vec<char>>>= Vec::new();

            // println!("Caben  {} Bins a lo ancho", amount_bins_ancho);
            // println!("Caben  {} Bins a lo alto", amount_bins_alto);

            //LLENAMOS EL ARREGLO DEL WORK AREA CON EL NUMERO DEL BIN PARA VIZUALIZARLOS
                //ESTO ES MERAMENTE ESTÉTICO, PARA SETS DE CONTENEDORES DE DISTINTOS TAMAÑOS LO EVITAREMOS
                // llenar_arreglo_con_bins(&mut wa_space_array, &bins, amount_bins_alto, amount_bins_ancho);
            
            //MOSTRAR BINS EN EL WORK AREA
            // mostrar_array(&wa_space_array, &w_a);
            

            //FUNCIONA - A PEDIR LOS ITEMS   //AHORA SE USAN LOS DE LA INSTANCIA
            //let mut items: Vec<Rec> = pedir_items(&bins, &w_a);
            let mut items: Vec<Rec> = items_rec;
            println!("Items sin ordenar:");
            imprimir_items(&items);
            println!("Ordenando items...");
            //ORDENAR ITEMS DE MAYOR A MENOR
            ordenar_items(&mut items);
            imprimir_items(&items);
            
            //COMENZAR A ALMACENAR LOS ITEMS EN LOS CONTENEDORES
            let items_acomodados:i32=colocar_items(&items, &bins, &mut bins_array, &inst);
            println!("Items insertados: {}", items_acomodados);
            //MOSTRAR LO HECHO EN PANTALLA -> PODEMOS IMPRIMIR INDIVIDUALMENTE CADA CONTENEDOR
            let cont_usados:i32 = contar_contenedores_usados(&mut bins_array, &bins);
            
            println!("Contenedores usados: {}", cont_usados); let mut i_bin =0;
            for bin in bins{
                println!("Contenedor: {}", i_bin+1);
                mostrar_array(&bins_array[i_bin], &bin);
                i_bin +=1;
            }

            if (items_acomodados as usize) < items.len() {
                println!("No se pudieron insertar todos los items");
            }
            //DEBERÍAMOS EXPORTAR LOS RESULTADOS DE ALGUNA MANERA
            //CONT | ITEM |INICIO: x_i, y_i | FINAL: x_f, y_f

        // }
        // else {
        //     println!("Los contenedores no pueden ser más grandes que el area de trabajo");
        // }
        // println!("Work_Area: {}, {}", w_a.alto, w_a.ancho);
        // for bin in bins {
        //     println!("Bins size: {}, {}", bin.alto, bin.ancho);
        // }

        
    }    
}
fn colocar_items(items: &Vec<Rec>, bins: &Vec<Rec>, bins_array: &mut Vec<Vec<Vec<char>>>, inst:&Instancia)->i32{
    //INICIALIZAMOS LOS BINS CON 0s
    let mut acomodados: i32 =0;
    for bin in bins{
        bins_array.push(inicializar_space_array(bin));
        // println!("Bins_array: {:?}", bins_array);
    }
    // let mut  i_bin =0;
    // for bin in bins{
    //     println!("Contenedor: {}", i_bin+1);
    //     mostrar_array(&bins_array[i_bin], &bin);
    //     i_bin +=1;
    // }
    //RECORREMOS LOS ITEMS Y ACOMODAMOS UNO POR UNO
    for i in 0..items.len() { 
        if acomodar(bins.clone(), bins_array, items, i, &inst) ==true{
            acomodados+=1;
        }
    }
    acomodados
}
fn acomodar(bins: Vec<Rec>, bins_array: &mut Vec<Vec<Vec<char>>>, items: &Vec<Rec>, indice: usize, inst: &Instancia) -> bool{
    let mut insertado: bool = false;
    let mut i_b: i32 = -1;
    for b in bins{//RECORREMOS LOS CONTENEDORES B ES EL CONTENEDOR
        i_b += 1;
        if insertado == true {  //SI YA FUE INSERTADO, NOS SALIMOS DE LOS CONTENEDORES
            break;
        }
        let mut disp : Vec<(usize, usize)> = Vec::new(); //ARREGLO PARA ALMACENAR LOS INDICES DISPONIBLES
        let mut contador:i32=0; //CONTADOR PARA SABER EL AREA DISPONIBLE
        for i in 0..b.alto as usize{ //FILAS
            for j in 0..b.ancho as usize{ //COLUMNAS
                //CONTAMOS ESPACIOS DISPONIBLES
                // println!("bin: {:?}", b);
                // println!("Bin_array: {:?}", bins_array[i_b]);
                // println!("i_b: {}, i: {}, j: {}", i_b, i, j);
                //APARENTEMENTE ESTÁN DESSINCRONIZADOS LOS BINS REC CON LA MATRIZ DE CARACTERES
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

        //RECORREMOS LOS ESPACIOS DISPONBIES, MENOS LOS ÚLTIMOS (AREA DEL RECTANGULO) PORQUE COMPARAMOS CADA I CON SUS (AREA DEL RECTANGULO) SIGUIENTES
        let (i_c, j_c) = disp[0];
        let n_ec: usize = (b.ancho as usize * i_c) + j_c;
        
        for i in n_ec..(b.area as usize - (items[indice].area as usize ) ){ //RECORREMOS LOS ESPACIOS DISPONBIES. CADA ITERACIÓN ES UN POSIBLE LUGAR DONDE PONER EL ITEM
            if insertado == true{ //SI YA FUE INSERTADO NOS SALIMOS DE LOS ESPACIOS DISPONIBLES
                break;
            }

            //println!("\nItem: {} | Contenedor: {} | Desde: {}:{} ",indice, b, i, n_ec);

            let mut contador_disp:i32=0;
            let mut coor_insert: Vec<(usize, usize)> = Vec::new();

            //COMPARAMOS CADA I CON (SUS AREA DEL RECTANGULO) SIGUIENTES
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
                    //print!("ec: {}, desde: {}, j: {}", ec, i, j_f64);

                    //NICE: AHORA NECESITAMOS QUE NOS GENERE LOS ESPACIOS DISPONIBLES A PARTIR DE 2 INDICES
                    //OBTENEMOS LOS 2 INDICES A PARTIR DEL NUMERO DE 1 INDICE 
                    
                    let i_comp:usize= (ec as f64 / len_f64).floor() as usize;    
                    let j_comp:usize= ec % len_f64 as usize;
                    //print!(" i_comp:{}, j_comp: {}\n", i_comp, j_comp);
                    //COMPROBAMOS QUE ESAS DIRECCIONES ESTÉN DISPONIBLES (0 es disponible)
                    if ec >= b.area as usize {break}
                    if bins_array[i_b as usize][i_comp][j_comp] == char::from_u32(48 as u32).unwrap(){
                        //PARA ESO NECESITAMOS UN CONTADOR
                        contador_disp+=1;
                        coor_insert.push((i_comp, j_comp));
                    }     
                }
                
            }// END FOR QUE GENERA LOS INDICES Y VERIFICA SU DISPONIBILIDAD

            //SI ESTAN DISPONIBLES LO INSERTAMOS
            if contador_disp >= items[indice].area{
                //INSERTAR
                for (i_comp, j_comp) in coor_insert.clone(){
                    let character = char::from_u32((64+indice) as u32).unwrap();
                    bins_array[i_b as usize][i_comp][j_comp] = character;
                }
                //SI LO INSERTAMOS, PONEMOS UNA VARIABLE BOOL DE INSERTADO
                let contenido: String = format!("item:{}, contenedor: {}, fila:{}, col: {}\n", (indice+1), (i_b+1), coor_insert[0].0, coor_insert[0].1);
                let mut archivo = OpenOptions::new().append(true).create(true).open(format!("sol-{}.txt", inst.titulo)).unwrap();
                archivo.write_all(contenido.as_bytes()).unwrap();
                insertado=true;
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
fn contar_contenedores_usados(bins_array: &mut Vec<Vec<Vec<char>>>, bins: &Vec<Rec>) -> i32{
    let mut cont_usados: i32 = 0;
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
    cont_usados
}
// fn pedir_items(bins: &Vec<Rec>, w_a: &Rec)->Vec<Rec>{
//     //CANTIDAD DE ITEMS
//     let mut sum_area:i32 = 0;
//     let mut area_valida: bool = false;
//     let mut items: Vec<Rec> = Vec::new();
//     while !area_valida {
//         let items_amout: i32 = get_size("cantidad de items".to_string());
//         //SI EL USUARIO INGRESÓ UNA CANTIDAD VALIDA DE ITEMS
//         if items_amout > 0 {
//             let mut i =0;
//             //PEDIMOS LOS ITEMS
//             while i < items_amout {
//                 let rec: Rec = obtener_rectangulo(( "rectangulo ".to_string()+&(i+1).to_string()+":" ).to_string());
//                 //SI EL ITEM CABE EN EL BIN SE AGREGA
//                 if rec.alto <= bins.alto && rec.ancho <= bins.ancho {
//                     items.push(rec.clone());
//                     sum_area+= rec.area;
//                     i+=1;
//                 }
//                 //SI NO, SE PIDE DE NUEVO
//                 else{
//                     println!("El alto y ancho del item debe ser menor que el de el contenedor");
//                 }
//             }  
//         }
//         if sum_area <= w_a.area {
//             area_valida=true;
//         }
//         else{
//             println!("La suma del área de los items no puede ser mayor que el área de trabajo");
//         }
//     }
//     items
// }
// fn llenar_arreglo_con_bins(wa_space_array: &mut Vec<Vec<char>>, bins: &Rec, amount_bins_alto:i32, amount_bins_ancho: i32){
//     let mut contador:u32=64;
//             for i in 0..amount_bins_alto {
//                 for j in 0..amount_bins_ancho { 

//                     contador+=1;
//                     for k in 0..bins.alto {
//                         let k_i:i32 = k+(bins.alto*i);
//                         for l in 0..bins.ancho { 
//                             let l_i:i32 = l+(bins.ancho*j);
                            
//                             wa_space_array[k_i as usize][l_i as usize] = char::from_u32(contador).unwrap();
        
//                         }              
//                     }       

//                 }              
//             }
// }
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
            print!("[{}]", array[i][j]);
        }                    
        println!("");
    }
}
// fn obtener_rectangulo(dato: String) -> Rec{
//     let mut w_a = Rec{
//         alto: get_size(("alto del ".to_string()+&dato).to_string()),
//         ancho: get_size(("ancho del ".to_string()+&dato).to_string()),
//         area: 0
//     };
//     w_a.obtener_area();
//     w_a
// }
// fn get_size(dato: String) -> i32{
//     let stdin = std::io::stdin();
//     loop{
//         let mut dato_ingresado = String::new();
//         println!("Ingresa tu {}", dato);
//         stdin.read_line(&mut dato_ingresado).unwrap();
//         dato_ingresado= dato_ingresado.trim().to_string();

//         if dato_ingresado.len() > 0
//         {
//             let i_dato_ingresado: i32 = match dato_ingresado.trim().parse(){
//                 Ok(num) => num, 
//                 Err(_)=> continue
//             };
//             if  i_dato_ingresado > 0
//             {
//                 break i_dato_ingresado
//             }

//         }

//     }

// }
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
        println!("H: {}, W: {}, A:{}", items[i].alto, items[i].ancho, items[i].area);
    }
}
