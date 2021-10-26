mod integration_test {
    use checkbom::mg_mod::*;
    #[test]
    fn test_bom_file_to_vec() {
        let config = Config {
            bom1: "tests/BOM1.csv".to_string(),
            bom2: "".to_string(),
            delim1: CsvDelim::Tab,
            delim2: CsvDelim::Zap,
        };
        let bom = bom_file_to_vec(&config.bom1, config.delim1);
        /*
        Odoo	Ref	Value	Name	Qnty
        1717	C56, 	1uF	C_1uF_25V_SMD_0603	1
        -	C57, C58, 	100pF	Chip ceramic capacitor 0603 100 pF 5% 50V C0G (101)- GRM1885C1H101JA01D [SMD]	2
        1813	C63,	22pF	C_22pF_50V_SMD_0603	1
        3354	C64, C72	100uF	Aluminum capacitors RVZ series 100uF 35V, 8X10mm, RVZ-35V101MGA5U-R2 [SMD]	2
        */
        /*
        pub struct BomVec {
            pub b_odoo: Option<usize>,
            pub b_ref: Vec<BomRef>,
            pub b_value: String,
            pub b_name: String,
            pub b_qnty: Option<usize>,
        }
        */
        let av = &bom.unwrap()[0..4];
        assert_eq!(Some(1717), av.get(0).unwrap().b_odoo);
        assert_eq!(None, av.get(1).unwrap().b_odoo);
        assert_eq!(Some(1813), av.get(2).unwrap().b_odoo);
        assert_eq!(Some(3354), av.get(3).unwrap().b_odoo);

        assert_eq!("C56", av.get(0).unwrap().b_ref.get(0).unwrap());
        assert_eq!("C63", av.get(2).unwrap().b_ref.get(0).unwrap());
        assert_eq!("C72", av.get(3).unwrap().b_ref.get(1).unwrap());
    }
}
