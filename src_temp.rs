//let contents = fs::read_to_string(config.bom1)?;
//let file = fs::File::open(config.bom1)?;
/*
        let mut rdr = csv::Reader::from_path(config.bom1)?;
        for result in rdr.records() {
            let record = result?;
            println!("{:?}", record);
        }
*/
/*let dat = "C1,C2,C3,C4".to_string();
let vs: Vec<&str> = dat.split(',').collect();
for i_x in &vs {
    println!("{}", i_x);
}*/


/*let mut ired = reader.records();
let iresa = ired.next().unwrap().unwrap();
let iresb: BomRows = iresa.deserialize(None)?;
if (iresb.b_odoo.is_some()) {
    println!("{}", iresb.b_odoo.unwrap());
} else {
    println!("-",);
}*/
//let mut wtr = csv::Writer::from_writer(vec![]);
let _wtr = csv::WriterBuilder::new()
    .delimiter(b'\t')
    .from_writer(vec![]);
//wtr_row.b_odoo = "df".to_string();
//let mut bom1_ref:

/*    let mut wtr_row = BomRowsWrt {
    b_odoo: String::from("-"),
    b_ref: String::from("-"),
    b_value: String::from("-"),
    b_name: String::from("-"),
    b_qnty: String::from("-"),
};*/


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(config.bom1)?;

    let mut bom_vec1: Vec<BomVec> = Vec::new();

    for result in reader.records() {
        // выкидывает без ошибки битые строки!

        if result.is_ok() {
            // строка в таблице без ошибок
            let r_strrec = result.expect("SrtingRecord error"); //Ошибка в строке
            let ires: BomRows = r_strrec
                .deserialize(None)
                .expect("Parser deserialize error"); // Ошибка в строке CSV
            let sd = String::from("-");
            if let Some(v) = ires.b_odoo {
                sd = format!("{}", v);
            };
            //    wtr_row.b_odoo = sd;
            //    wtr_row.b_ref = ires.b_ref.to_string();
            let wtr_row = ires.b_ref.to_string();
            //println!("{}", wtr_row);
            /*let mut wtr_row: String = "-".to_string();
            if ires.b_ref.len() > 2 {
                wtr_row = ires.b_ref[0..ires.b_ref.len() - 2].to_string();
            }
            println!("{}", wtr_row);*/

            let mut list_ref: Vec<String> = wtr_row
                .split(',')
                .map(|c| c.chars().filter(|c| !c.is_whitespace()).collect())
                .collect();
            if let Some(v) = list_ref.last() {
                //    println!("{}, {}", v.len(), v);
                if v.eq("") {
                    list_ref.pop();
                }
            }

            //wtr.serialize(wtr_row)?;
            //    println!("{}\t{}", wtr_row.b_odoo, wtr_row.b_ref);
            println!("{}", sd);
            for i_x in &list_ref {
                //let s_x: String = i_x.chars().filter(|c| !c.is_whitespace()).collect();
                println!("{}", i_x);
            }
        }
        //    println!("{:?}", result?);
    }
    //let mm = wtr.into_inner()?;
    //let written = String::from_utf8(mm)?;
    //println!("{}", written);

    Ok(())
}




// строку ref преобразуем в вектор
/*let wtr_row = ires.b_ref.to_string();
let mut list_ref: Vec<String> = wtr_row
    .split(',')
    .map(|c| c.chars().filter(|c| !c.is_whitespace()).collect())
    .collect();
if let Some(v) = list_ref.last() {
    if v.eq("") {
        list_ref.pop();
    }
}

// преобразуем вектор ref в вектор BomRef
let mut list_br_ref: Vec<BomRef> = Vec::new();
for i_x in &list_ref {
    list_br_ref.push(BomRef {
        br_ref: i_x.to_owned(),
        br_checked: false,
    });
}
*/


/*
let mut reader = csv::ReaderBuilder::new()
    .delimiter(b'\t')
    .from_path(config.bom1)?;

let mut bom_vec1: Vec<BomVec> = Vec::new();

for result in reader.records() {
    // выкидывает без ошибки битые строки!

    if result.is_ok() {
        // строка в таблице без ошибок
        let r_strrec = result.expect("SrtingRecord error"); //Ошибка в строке
        let ires: BomRows = r_strrec
            .deserialize(None)
            .expect("Parser deserialize error"); // Ошибка в строке CSV

        let mut sd = String::from("-");
        // проверяем наличие одоо кода
        if let Some(v) = ires.b_odoo {
            sd = format!("{}", v);
        };

        // преобразуем строку b_ref в вектор Vec<BomRef>.
        // C1, C2, C3, C4, -> {C1, false}, {C2, false}, {C3, false}...
        let wtr_row2 = ires.b_ref.to_string();
        let mut list_ref2: Vec<BomRef> = wtr_row2
            .split(',')
            .map(|c| BomRef {
                br_ref: c.chars().filter(|c| !c.is_whitespace()).collect(),
                br_checked: false,
            })
            .collect();
        // послений элемент как правило пустой от парсера, его удаляем
        if let Some(v) = list_ref2.last() {
            if v.br_ref.eq("") {
                list_ref2.pop();
            }
        }

        // сохраняем строку csv как строку вектора
        let a_v = BomVec {
            b_odoo: ires.b_odoo,
            b_ref: list_ref2, //list_br_ref,
            b_value: ires.b_value.to_owned(),
            b_name: ires.b_name.to_owned(),
            b_qnty: ires.b_qnty,
        };
        bom_vec1.push(a_v);

        // печатаем для отладки одоо + ref
        println!("{}", sd);
        if let Some(v) = bom_vec1.get(bom_vec1.len() - 1) {
            for i_x in &v.b_ref {
                print!("{},", i_x.br_ref);
            }
        }
        println!("");
    }
}

Ok(())
*/

#[cfg(test)]
mod module_test {
    use mg_lib::mg_mod;
    #[test]
    /*fn test__bom_file_to_vec() {
        let config: Config ={
            bom1: "src/test/BOM1.csv".to_string(),
            bom2: "",
            delim1: Tab,
            delim2
        }
        //assert_eq!(hw_lib1::addz(3, 2), 5);
    }
    */
}
