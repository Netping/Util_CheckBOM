#[allow(dead_code)] // отключаем предупреждение о неиспользуемом коде
pub mod mg_mod {
    //use csv::*;
    use std::env;
    use std::error::Error;
    //use std::fs;
    use serde::Deserialize;
    //use serde::Serialize;
    extern crate csv;
    use std::collections::HashMap;
    use std::collections::HashSet;
    //    use std::iter::FromIterator;
    use std::process;
    use std::str;

    #[derive(PartialEq)]
    pub enum CsvDelim {
        Tab,
        Zap,
    }

    pub struct Config {
        pub bom1: String,
        pub bom2: String,
        pub delim1: CsvDelim,
        pub delim2: CsvDelim,
    }

    impl Config {
        /*pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        */
        pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
            args.next();

            match args.len() {
                //1 => return Err("Must be 3 arguments. Try h for help."),
                1 => {
                    let help = match args.next() {
                        Some(arg) => arg,
                        None => return Err("Unknown arguments. Try h for help."),
                    };
                    if help.contains("h") {
                        let help_dat = "
checkbom - утилита для проверки BOM2 по BOM1.
BOM1 - созданный схемотехниками
BOM2 - созданные снабжением и выгруженный из odoo БД.
формат BOM - csv разделитель tab или ,
формат полей BOM: Odoo,Ref,Value,Name,Qnty
checkbom tz filebom1.csv filebom2.CSV
tz - разделитель tab для filebom1.csv и , для filebom2.CSV
варианты использвоания tt, zt, zz ";
                        println!("{}", help_dat);
                        process::exit(1);
                    } else {
                        return Err("Unknown arguments. Try h for help.");
                    }
                }
                2 => return Err("Must be 3 arguments. Try h for help."),
                3 => println!("checkbom starting"),
                _ => return Err("Unknown arguments. Try h for help."),
            }

            let delim = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a delimiters file"),
            };
            let delim1: CsvDelim;
            let delim2: CsvDelim;
            if delim.len() != 2 {
                return Err("Unknown delimiters arguments. Try h for help.");
            } else {
                let mut i_d = delim.chars();
                let i_c = i_d.next().unwrap_or_else(|| '-');
                match i_c {
                    't' => delim1 = CsvDelim::Tab,
                    'z' => delim1 = CsvDelim::Zap,
                    _ => return Err("Unknown delimiters arguments. Try h for help."),
                }
                let i_c = i_d.next().unwrap_or_else(|| '-');
                match i_c {
                    't' => delim2 = CsvDelim::Tab,
                    'z' => delim2 = CsvDelim::Zap,
                    _ => return Err("Unknown delimiters arguments. Try h for help."),
                }
            }
            let bom1 = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a BOM1 file"),
            };
            let bom2 = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a BOM2 file"),
            };

            Ok(Config {
                bom1,
                bom2,
                delim1,
                delim2,
            })
            //Ok(Config { query, filename })
        }
    }

    #[derive(Deserialize)]
    pub struct BomRows<'a> {
        #[serde(deserialize_with = "csv::invalid_option")]
        b_odoo: Option<usize>,
        b_ref: &'a str,
        b_value: &'a str,
        b_name: &'a str,
        b_qnty: &'a str,
    }

    #[derive(Debug, PartialEq)]
    pub struct BomVec {
        pub b_odoo: Option<usize>,
        pub b_ref: Vec<String>,
        pub b_value: String,
        pub b_name: String,
        pub b_qnty: Option<usize>,
    }
    /*
    #[derive(Debug, PartialEq)]
    pub struct BomHash<'a> {
        pub b_odoo: Option<usize>,
        pub b_ref: HashSet<&'a str>,
        pub b_value: String,
        pub b_name: String,
        pub b_qnty: Option<usize>,
    } */

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let bom1 = bom_file_to_vec(&config.bom1, config.delim1)?;
        let bom2 = bom_file_to_vec(&config.bom2, config.delim2)?;

        let (aerr, ahash) = boms_to_hash(&bom1, &bom2);

        if aerr != 0 {
            println!("Ref check fail: {} Errors", aerr);
            return Ok(());
        } else {
            println!("Check Refs in both files complete");
        }
        let aerr = check_ref_to_name(&ahash);
        if aerr != 0 {
            println!("Names check fail: {} Errors", aerr);
            return Ok(());
        } else {
            println!("Check Names complete");
        }

        let aerr = check_odoo(&ahash);
        if aerr != 0 {
            println!("Odoo check fail: {} Errors", aerr);
            return Ok(());
        } else {
            println!("Check Odoo complete");
        }

        let aerr = check_qnty(&ahash);
        if aerr != 0 {
            println!("Qnty check fail: {} Errors", aerr);
            return Ok(());
        } else {
            println!("Check Qnty complete");
        }

        print_final_table(&ahash);

        Ok(())
    }

    fn print_final_table(hbs: &HashBoms) {
        println!("Odoo\tRef\tValue\tName\tQnty\tOdoo2\tRef2\tValue2\tName2\tQnty2");
        let mut prd1: HashSet<&str> = HashSet::<&str>::new();
        let mut prd2: HashSet<&str> = HashSet::<&str>::new();

        for n_bv in &hbs.name1_hm {
            for r_i in n_bv.1 {
                if !prd1.contains(r_i) {
                    // ref not printered
                    let br1 = hbs.bom1_hm.get(r_i).unwrap();
                    let br2 = hbs.bom2_hm.get(r_i).unwrap();
                    if !prd2.contains(r_i) {
                        print_bomvec("", "", Some(br1), Some(br2));
                    } else {
                        print_bomvec("", "", Some(br1), None);
                    }
                    prd1.extend(br1.b_ref.iter().map(|c| c.as_str()));
                    prd2.extend(br2.b_ref.iter().map(|c| c.as_str()));
                } else {
                    if !prd2.contains(r_i) {
                        let br2 = hbs.bom2_hm.get(r_i).unwrap();
                        print_bomvec("", "", None, Some(br2));
                        prd2.extend(br2.b_ref.iter().map(|c| c.as_str()));
                    }
                }
            }
        }
        if hbs.name1bv_dnphm.len() > 0 {
            println!("DNP in BOM1:");
            for ni in &hbs.name1bv_dnphm {
                for xi in ni.1 {
                    print_bomvec("", "", Some(xi), None);
                }
            }
        }
        if hbs.name2bv_dnphm.len() > 0 {
            println!("DNP in BOM2:");
            for ni in &hbs.name2bv_dnphm {
                for xi in ni.1 {
                    print_bomvec("", "", Some(xi), None);
                }
            }
        }
    }

    fn check_qnty(hbs: &HashBoms) -> usize {
        let mut err: usize = 0;

        for n_bv in &hbs.name1_bvhm {
            let mut qn1 = 0;
            let mut qn2 = 0;
            let name2 = &hbs.bom2_hm[n_bv.1[0].b_ref[0].as_str()].b_name;
            let mut fe = false;
            let mut fe2 = false;
            //n_bv.1[0].b_ref[0];
            for bv_i in n_bv.1 {
                if bv_i.b_qnty.is_some() {
                    qn1 += bv_i.b_qnty.unwrap();
                    let qn1r = bv_i.b_ref.len();
                    if bv_i.b_qnty.unwrap() != qn1r {
                        println!("Error BOM1. Ref and Qnty is not same");
                        print_bomvec("", "", Some(bv_i), None);
                        err += 1;
                        fe = true;
                    }
                } else {
                    // ошибка количество не существует
                    println!("Error BOM1. Qnty is not exist");
                    print_bomvec("", "", Some(bv_i), None);
                    err += 1;
                    fe = true;
                }
            }
            let am = hbs.name2_bvhm.get(name2.as_str()).unwrap();
            for bv_i in am {
                if bv_i.b_qnty.is_some() {
                    qn2 += bv_i.b_qnty.unwrap();
                    let qn2r = bv_i.b_ref.len();
                    if bv_i.b_qnty.unwrap() != qn2r {
                        println!("Error BOM2. Ref and Qnty is not same");
                        print_bomvec("", "", Some(bv_i), None);
                        err += 1;
                        fe2 = true;
                    }
                } else {
                    // ошибка количество не существует
                    println!("Error BOM2. Qnty is not exist");
                    print_bomvec("", "", Some(bv_i), None);
                    err += 1;
                    fe2 = true;
                }
            }

            if !fe && !fe2 {
                if qn1 != qn2 {
                    println!("Error Qnty in BOM1 and BOM2 is not same");
                    println!("BOM1:");
                    println!("{}", n_bv.0);
                    for a in n_bv.1.iter() {
                        print_bomvec("", "", Some(a), None);
                    }
                    println!("BOM2:");
                    println!("{}", name2);
                    for a in am.iter() {
                        print_bomvec("", "", Some(a), None);
                    }
                    err += 1;
                }
            }
        }

        err
    }

    fn check_odoo(hbs: &HashBoms) -> usize {
        let mut err: usize = 0;
        for n_bv in &hbs.name1_bvhm {
            let mut pr_odoo = n_bv.1[0].b_odoo;
            let mut pr_bv = n_bv.1[0];
            let mut wrn = false;
            for bv_i in n_bv.1 {
                if bv_i.b_odoo != pr_odoo {
                    if bv_i.b_odoo.is_some() && pr_odoo.is_some() {
                        // разные odoo
                        err += 1;
                        println!(
                            "Error BOM1. Odoo is not same: {} and {}",
                            bv_i.b_odoo.unwrap(),
                            pr_odoo.unwrap()
                        );
                        print_bomvec("", "", Some(pr_bv), None);
                        print_bomvec("", "", Some(bv_i), None);
                    } else {
                        // один код пустой второй значение
                        wrn = true;
                    }
                }
                if pr_odoo.is_none() {
                    pr_odoo = bv_i.b_odoo;
                    pr_bv = bv_i;
                }
            }
            if wrn {
                println!("Warning BOM1 Some and None Odoo:");
                for bv_i in n_bv.1 {
                    print_bomvec("", "", Some(bv_i), None);
                }
            }
        }

        for n_bv in &hbs.name2_bvhm {
            let mut pr_odoo = n_bv.1[0].b_odoo;
            let mut pr_bv = n_bv.1[0];
            let mut wrn = false;
            for bv_i in n_bv.1 {
                if bv_i.b_odoo != pr_odoo {
                    if bv_i.b_odoo.is_some() && pr_odoo.is_some() {
                        // разные odoo
                        err += 1;
                        println!(
                            "Error BOM2. Odoo is not same: {} and {}",
                            bv_i.b_odoo.unwrap(),
                            pr_odoo.unwrap()
                        );
                        print_bomvec("", "", Some(pr_bv), None);
                        print_bomvec("", "", Some(bv_i), None);
                    } else {
                        // один код пустой второй значение
                        wrn = true;
                    }
                }
                if pr_odoo.is_none() {
                    pr_odoo = bv_i.b_odoo;
                    pr_bv = bv_i;
                }
            }
            if wrn {
                println!("Warning BOM2 Some and None Odoo:");
                for bv_i in n_bv.1 {
                    print_bomvec("", "", Some(bv_i), None);
                }
            }
        }

        if err == 0 {
            for n_i in &hbs.name1_hm {
                let mut odoo1: Option<usize> = None;
                let mut odoo2: Option<usize> = None;
                let mut r_ierr = false;
                for r_i in n_i.1 {
                    if odoo1.is_none() {
                        odoo1 = hbs.bom1_hm.get(r_i).unwrap().b_odoo;
                    }
                    if odoo2.is_none() {
                        odoo2 = hbs.bom2_hm.get(r_i).unwrap().b_odoo;
                    }
                    if odoo1.is_some() && odoo2.is_some() {
                        if (odoo1 != odoo2) && !r_ierr {
                            // Разные одоо коды
                            r_ierr = true;
                            err += 1;
                            println!(
                                "Error. Odoos is not same: {} and {}",
                                odoo1.unwrap(),
                                odoo2.unwrap()
                            );
                            println!("BOM1:");
                            print_bomvec("", "", hbs.bom1_hm.get(r_i).copied(), None);
                            println!("BOM2:");
                            print_bomvec("", "", hbs.bom2_hm.get(r_i).copied(), None);
                        }
                    }
                }
            }
        }
        err
    }

    fn check_ref_to_name(hbs: &HashBoms) -> usize {
        //C1, C14, C38,
        let mut err: usize = 0;
        for n_i in &hbs.name1_hm {
            let mut f2ref: String = "".to_string();
            let mut fen = true;
            let mut bv: Option<&BomVec> = None;
            //    println!("");
            for r_i in n_i.1 {
                //    print!("{},", r_i);
                if fen {
                    fen = false;
                    f2ref.push_str(&hbs.bom2_hm.get(r_i).unwrap().b_name);
                    bv = hbs.bom2_hm.get(r_i).copied();
                }
                //bom2_hm.get(r_i).unwrap().b_name.contains(f2ref) ;
                if !hbs.bom2_hm.get(r_i).unwrap().b_name.contains(&f2ref)
                    || (hbs.bom2_hm.get(r_i).unwrap().b_name.len() != f2ref.len())
                {
                    // error not same names Ref in bom1 and bom2
                    println!("Error. Names for Ref:{} is not same", r_i);
                    println!("BOM1:");
                    print_bomvec("", "", hbs.bom1_hm.get(r_i).copied(), None);
                    println!("BOM2:");
                    print_bomvec("", "", hbs.bom2_hm.get(r_i).copied(), None);
                    //println!("{}", f2ref);
                    print_bomvec("", "", bv, None);
                    err += 1;
                }
            }
        }
        for n_i in &hbs.name2_hm {
            let mut f2ref: String = "".to_string();
            let mut fen = true;
            let mut bv: Option<&BomVec> = None;
            //println!("");
            for r_i in n_i.1 {
                //    print!("{},", r_i);
                if fen {
                    fen = false;
                    f2ref.push_str(&hbs.bom1_hm.get(r_i).unwrap().b_name);
                    bv = hbs.bom1_hm.get(r_i).copied();
                }
                //bom2_hm.get(r_i).unwrap().b_name.contains(f2ref) ;
                if !hbs.bom1_hm.get(r_i).unwrap().b_name.contains(&f2ref)
                    || (hbs.bom1_hm.get(r_i).unwrap().b_name.len() != f2ref.len())
                {
                    // error not same names Ref in bom1 and bom2
                    println!("Error. Names for Ref:{} is not same", r_i);
                    println!("BOM2:");
                    print_bomvec("", "", hbs.bom2_hm.get(r_i).copied(), None);
                    println!("BOM1:");
                    print_bomvec("", "", hbs.bom1_hm.get(r_i).copied(), None);
                    //println!("{}", f2ref);
                    print_bomvec("", "", bv, None);
                    err += 1;
                }
            }
        }
        err
    }
    /*
    fn check_uniq_ref(bom1: &Vec<BomVec>, bom2: &Vec<BomVec>) -> impl Iterator<Item = BomRowsWrt> {
        let mut ai: Vec<BomRowsWrt> = Vec::new();
        let mut ref1 = HashMap::new();
        for b_i in bom1.iter() {
            for r_i in b_i.b_ref.iter() {
                if ref1.contains(r_i) {

                }
            }
        }
        ai.into_iter()
    } */
    /*    pub fn cmp_boms<'a>(
            bom1: &'a Vec<BomVec>,
            bom2: &'a Vec<BomVec>,
        ) ->  <Iterator<Item = &'a str> + 'a> {
        }
    */
    struct HashBoms<'a> {
        bom1_hm: HashMap<&'a str, &'a BomVec>,
        bom2_hm: HashMap<&'a str, &'a BomVec>,
        name1_hm: HashMap<&'a str, HashSet<&'a str>>,
        name2_hm: HashMap<&'a str, HashSet<&'a str>>,
        name1_bvhm: HashMap<&'a str, Vec<&'a BomVec>>,
        name2_bvhm: HashMap<&'a str, Vec<&'a BomVec>>,
        name1bv_dnphm: HashMap<&'a str, Vec<&'a BomVec>>,
        name2bv_dnphm: HashMap<&'a str, Vec<&'a BomVec>>,
    }
    //https://gist.github.com/conundrumer/4e57c14705055bb2deac1b9fde84f83b
    fn boms_to_hash<'a>(bom1: &'a Vec<BomVec>, bom2: &'a Vec<BomVec>) -> (usize, HashBoms<'a>) {
        //    let mut ai: Vec<BomRowsWrt> = Vec::new();
        let mut bom1hm: HashMap<&str, &BomVec> = HashMap::new(); // Ref BomVec
        let mut bom2hm: HashMap<&str, &BomVec> = HashMap::new(); //
        let mut name1hm: HashMap<&str, HashSet<&str>> = HashMap::new(); // <Name, HS<Ref>>
        let mut name2hm: HashMap<&str, HashSet<&str>> = HashMap::new();
        let mut name1bvhm: HashMap<&str, Vec<&BomVec>> = HashMap::new(); // Name Vec<BomVec>
        let mut name2bvhm: HashMap<&str, Vec<&BomVec>> = HashMap::new();
        let mut name1bv_dnp_hm: HashMap<&str, Vec<&BomVec>> = HashMap::new(); // Name Vec<BomVec>
        let mut name2bv_dnp_hm: HashMap<&str, Vec<&BomVec>> = HashMap::new(); // Name Vec<BomVec>

        let mut ch_error: usize = 0;
        for b_i in bom1.iter() {
            // сохздем хешкарту и проверяем уникальность ref
            if !b_i.b_value.contains("DNP") {
                for r_i in b_i.b_ref.iter() {
                    let mut erb = false;
                    if r_i.len() < 1 {
                        println!("BOM1 error. Empty or short Ref:{} here:", r_i);
                        print_bomvec("", "", Some(b_i), None);
                    }
                    bom1hm
                        .entry(r_i.as_str())
                        .and_modify(|_| {
                            //ai.push(copy_bom1_to_emrow(format!("Doublicate Ref:{}", r_i), b_i));
                            println!("BOM1 error. Doublicate Ref:{} here:", r_i);
                            print_bomvec("", "", Some(b_i), None);
                            ch_error += 1;
                            erb = true;
                        })
                        .or_insert(b_i);
                    if erb {
                        print_bomvec("", "", Some(bom1hm.get(r_i.as_str()).unwrap()), None);
                    }
                }
                name1bvhm
                    .entry(&b_i.b_name)
                    .and_modify(|av| {
                        if name1hm.get(&b_i.b_name.as_str()).is_none() {
                            av.push(&b_i)
                        } else {
                            if !name1hm
                                .get(&b_i.b_name.as_str())
                                .unwrap()
                                .contains(b_i.b_ref.get(0).unwrap_or(&"".to_string()).as_str())
                            {
                                av.push(&b_i)
                            }
                        };
                    })
                    .or_insert_with(|| {
                        let mut avc = Vec::<&BomVec>::new();
                        avc.push(&b_i);
                        avc
                    });
                name1hm
                    .entry(&b_i.b_name)
                    .or_insert_with(|| HashSet::<&str>::new())
                    .extend(b_i.b_ref.iter().map(|c| c.as_str()));
            } else {
                // DNP
                name1bv_dnp_hm
                    .entry(&b_i.b_name)
                    .and_modify(|av| av.push(&b_i))
                    .or_insert_with(|| {
                        let mut avc = Vec::<&BomVec>::new();
                        avc.push(&b_i);
                        avc
                    });
            }
        }
        for b_i in bom2.iter() {
            if !b_i.b_value.contains("DNP") {
                for r_i in b_i.b_ref.iter() {
                    let mut erb = false;
                    if r_i.len() < 1 {
                        println!("BOM2 error. Empty or short Ref:{} here:", r_i);
                        print_bomvec("", "", Some(b_i), None);
                    }
                    if bom2hm.contains_key(r_i.as_str()) {
                        //ai.push(copy_bom2_to_emrow(format!("Doublicate Ref:{}", r_i), b_i));
                        println!("BOM2 error. Doublicate Ref:{} here:", r_i);
                        print_bomvec("", "", Some(b_i), None);
                        ch_error += 1;
                        erb = true;
                    } else {
                        bom2hm.insert(r_i.as_str(), b_i);
                    }
                    if erb {
                        print_bomvec("", "", Some(bom1hm.get(r_i.as_str()).unwrap()), None);
                    }
                }
                name2bvhm
                    .entry(&b_i.b_name)
                    .and_modify(|av| {
                        if name2hm.get(&b_i.b_name.as_str()).is_none() {
                            av.push(&b_i)
                        } else {
                            if !name2hm
                                .get(&b_i.b_name.as_str())
                                .unwrap()
                                .contains(b_i.b_ref.get(0).unwrap_or(&"".to_string()).as_str())
                            {
                                av.push(&b_i)
                            }
                        };
                    })
                    .or_insert_with(|| {
                        let mut avc = Vec::<&BomVec>::new();
                        avc.push(&b_i);
                        avc
                    });
                name2hm
                    .entry(&b_i.b_name)
                    .or_insert_with(|| HashSet::<&str>::new())
                    .extend(b_i.b_ref.iter().map(|c| c.as_str()));
            } else {
                // DNP
                name2bv_dnp_hm
                    .entry(&b_i.b_name)
                    .and_modify(|av| av.push(&b_i))
                    .or_insert_with(|| {
                        let mut avc = Vec::<&BomVec>::new();
                        avc.push(&b_i);
                        avc
                    });
            }
        }

        let slh = &bom1hm;
        for ref_i in slh {
            // проверяем что ref есть в обоих бомах
            if !bom2hm.contains_key(ref_i.0) {
                ch_error += 1;
                //ai.push(copy_bom1_to_emrow(
                //    format!("Bom2 do not contains Ref:{}", ref_i.0),
                //        ref_i.1,
                //));
                println!("BOM2 error. Do not contains Ref:{} from:", ref_i.0);
                print_bomvec("", "", Some(ref_i.1), None);
            }
        }
        let slh = &bom2hm;
        for ref_i in slh {
            if !bom1hm.contains_key(ref_i.0) {
                ch_error += 1;
                //ai.push(copy_bom2_to_emrow(
                //format!("Bom1 do not contains Ref:{}", ref_i.0),
                //ref_i.1,
                //));
                println!("BOM1 error. Do not contains Ref:{} from:", ref_i.0);
                print_bomvec("", "", Some(ref_i.1), None);
            }
        }
        let hbs = HashBoms {
            bom1_hm: bom1hm,
            bom2_hm: bom2hm,
            name1_hm: name1hm,
            name2_hm: name2hm,
            name1_bvhm: name1bvhm,
            name2_bvhm: name2bvhm,
            name1bv_dnphm: name1bv_dnp_hm,
            name2bv_dnphm: name2bv_dnp_hm,
        };
        (ch_error, hbs)
    }

    #[test]
    pub fn test_boms_to_hash() {
        let mut bom1: Vec<BomVec> = Vec::new();
        let mut bom2: Vec<BomVec> = Vec::new();
        let ins = BomVec {
            b_odoo: Some(1234),
            b_ref: vec!["C1".to_string(), "C2".to_string(), "C3".to_string()],
            b_value: "100n".to_string(),
            b_name: "capacitor 0,1uF ceramic SMD_0603".to_string(),
            b_qnty: Some(3),
        };
        bom1.push(ins);
        let ins = BomVec {
            b_odoo: Some(1234),
            b_ref: vec!["C1".to_string(), "C2".to_string(), "C3".to_string()],
            b_value: "100n".to_string(),
            b_name: "capacitor 0,1uF ceramic murata SMD_0603".to_string(),
            b_qnty: Some(3),
        };
        bom2.push(ins);
        let (ae1, as1) = boms_to_hash(&bom1, &bom2);
        assert_eq!(ae1, 0);
        assert_eq!(as1.bom1_hm["C1"].b_odoo.unwrap(), 1234);
        assert_eq!(as1.bom1_hm["C3"].b_name, "capacitor 0,1uF ceramic SMD_0603");
        assert_eq!(as1.bom2_hm["C2"].b_qnty.unwrap(), 3);
        let ats = &as1.name1_bvhm["capacitor 0,1uF ceramic SMD_0603"];
        let ats2 = ats[0].b_odoo.unwrap();
        //    println!("bom1_hm name {}", ats2.b_name);
        assert_eq!(ats2, 1234);
        assert_eq!(ats[0].b_ref[1], "C2");

        /*
        pub struct BomVec {
            pub b_odoo: Option<usize>,
            pub b_ref: Vec<String>,
            pub b_value: String,
            pub b_name: String,
            pub b_qnty: Option<usize>,
        }*/
        /*fn boms_to_hash<'a>(
        bom1: &'a Vec<BomVec>,
        bom2: &'a Vec<BomVec>,
        ai: &mut Vec<BomRowsWrt>, */
    }

    /*
    struct BomRowsWrtType(Vec<BomRowsWrt>);
    impl AddRow for BomRowsWrtType {
        fn add_row(&self, x: usize) {
            //self.0.push(BomRowsWrt{...all fields});
        }
    }
    pub trait AddRow {
        fn add_row(&self, x: usize);
    }*/

    /*
    pub trait AddRow<Vec> {
        fn add_row(&self) {
            //self.push(BomRowsWrt{...all fields});
        }
    }
    impl AddRow<BomRowsWrt> for Vec<BomRowsWrt> {} */

    fn print_bomvec(mes1: &str, mes2: &str, obom1: Option<&BomVec>, obom2: Option<&BomVec>) {
        //Vec<BomVec>
        let mut p_s: String = "".to_string();
        if obom1.is_some() {
            let bom1 = obom1.unwrap();
            let mut sd = String::from("-\t");
            // проверяем наличие одоо кода
            if let Some(v) = bom1.b_odoo {
                sd = format!("{}\t", v);
            };
            p_s.push_str(&sd);

            if mes1.len() == 0 {
                for p_ref in &bom1.b_ref {
                    p_s.push_str(&format!("{}, ", p_ref));
                }
            }
            p_s.push_str("\t");
            p_s.push_str(&format!("{}\t", bom1.b_value));
            p_s.push_str(&format!("{}\t", bom1.b_name));

            let mut sd = String::from("-\t");
            // проверяем наличие одоо кода
            if let Some(v) = bom1.b_qnty {
                sd = format!("{}\t", v);
            };
            p_s.push_str(&sd);
        } else {
            p_s.push_str(&format!("\t{}\t\t\t\t", mes1));
        }

        if obom2.is_some() {
            let bom2 = obom2.unwrap();
            let mut sd = String::from("-\t");
            // проверяем наличие одоо кода
            if let Some(v) = bom2.b_odoo {
                sd = format!("{}\t", v);
            };
            p_s.push_str(&sd);

            if mes2.len() == 0 {
                for p_ref in &bom2.b_ref {
                    p_s.push_str(&format!("{}, ", p_ref));
                }
            }
            p_s.push_str("\t");
            p_s.push_str(&format!("{}\t", bom2.b_value));
            p_s.push_str(&format!("{}\t", bom2.b_name));

            let mut sd = String::from("-\t");
            // проверяем наличие одоо кода
            if let Some(v) = bom2.b_qnty {
                sd = format!("{}\t", v);
            };
            p_s.push_str(&sd);
        } else {
            p_s.push_str(&format!("\t{}\t\t\t\t", mes2));
        }

        println!("{}", p_s);
    }

    pub fn bom_file_to_vec(
        file_path: &String,
        dl: CsvDelim,
    ) -> Result<Vec<BomVec>, Box<dyn Error>> {
        let mut dlf = b'\t';
        if dl == CsvDelim::Zap {
            dlf = b',';
        }
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(dlf)
            .from_path(file_path)?;

        let mut bom_vec1: Vec<BomVec> = Vec::new();

        for result in reader.records() {
            // выкидывает без ошибки битые строки!

            if result.is_ok() {
                // строка в таблице без ошибок
                let r_strrec = result.expect("SrtingRecord error"); //Ошибка в строке
                let ires: BomRows = r_strrec
                    .deserialize(None)
                    .expect("Parser deserialize error"); // Ошибка в строке CSV

                //let mut sd = String::from("-");
                // проверяем наличие одоо кода
                //if let Some(v) = ires.b_odoo {
                //    sd = format!("{}", v);
                //};
                if !ires.b_ref.contains("Ref") {
                    let mut qnty_s = ires.b_qnty;
                    //println!("{}", qnty_s);
                    let mut qnty: Option<usize> = None;
                    if qnty_s.contains(".0") {
                        if qnty_s.len() > 2 {
                            qnty_s = &qnty_s[0..qnty_s.len() - 2];
                        }
                    }
                    let qnty_r = qnty_s.parse::<usize>();
                    if qnty_r.is_ok() {
                        qnty = Some(qnty_r.unwrap());
                    }
                    // преобразуем строку b_ref в вектор Vec<BomRef>.
                    // C1, C2, C3, C4, -> {C1, false}, {C2, false}, {C3, false}...
                    let wtr_row2 = ires.b_ref.to_string();
                    if wtr_row2.len() > 1 {
                        let mut list_ref2: Vec<String> = wtr_row2
                            .split(',')
                            .map(|c| c.chars().filter(|c| !c.is_whitespace()).collect())
                            .collect();
                        // послений элемент как правило пустой от парсера, его удаляем
                        if let Some(v) = list_ref2.last() {
                            if v.eq("") {
                                list_ref2.pop();
                            }
                        }

                        // сохраняем строку csv как строку вектора
                        let a_v = BomVec {
                            b_odoo: ires.b_odoo,
                            b_ref: list_ref2, //list_br_ref,
                            b_value: ires.b_value.to_owned(),
                            b_name: ires.b_name.to_owned(),
                            b_qnty: qnty,
                        };
                        bom_vec1.push(a_v);
                    } else {
                        println!("Warning! Ref is empty. Line ignored");
                        println!("In file: {}", file_path);
                        println!(
                            "{}\t{}\t{}\t{}\t{}",
                            ires.b_odoo.unwrap_or(0),
                            ires.b_ref,
                            ires.b_value,
                            ires.b_name,
                            qnty.unwrap_or(0)
                        );
                    }
                }

                // печатаем для отладки одоо + ref
                /*println!("{}", sd);
                if let Some(v) = bom_vec1.get(bom_vec1.len() - 1) {
                    for i_x in &v.b_ref {
                        print!("{},", i_x);
                    }
                }
                println!("");
                */
            }
        }

        Ok(bom_vec1)
    }

    pub fn search7<'query, 'a: 'query>(
        query: &'query str,
        contents: &'a str,
    ) -> impl Iterator<Item = (usize, &'a str)> + 'query {
        contents
            .lines()
            .enumerate()
            .filter(move |line| line.1.contains(query))
    }
}

#[cfg(test)]
mod tests {
    /*
    use super::mg_mod::*;
    #[test]
    fn get_emrow_test() {
        get_emrow();
        //assert_eq!(hw_lib1::addz(3, 2), 5);
    }
    */
}
