#[test]
fn hello_world() {
    println!("Hello World!");
}

#[test]
fn comments() {
    /*
    普通注释，其内容将被编译器忽略掉：
        // 单行注释，注释内容直到行尾。
        /* 块注释，注释内容一直到结束分隔符。 */

    文档注释，其内容将被解析成 HTML 帮助文档：
        /// 为接下来的项生成帮助文档。
        //! 为注释所属于的项（译注：如 crate、模块或函数）生成帮助文档。
    */
}

#[test]
fn formatted_print() {
    println!("there are {} or {} days in Febuary", 28, 29);
    println!("{S} {V} {O}", S = "I", V = "write", O = "codes");
    println!("{0} {2} {1}", "I", "write", "codes");

    println!("Deciaml       1024 --> (10) {}   ", 1024);
    println!("Binary        1024 --> (2)  {:b} ", 1024); // 100_0000_0000
    println!("Octal         1024 --> (8)  {:o} ", 1024); // 2000
    println!("Hexadeciaml   1024 --> (16) {:x} ", 1024); // 400

    println!("{x:>5}", x = 2); //  "    2"
    println!("{x:0>5}", x = 3); // "00003"
    println!("{x:0<5}", x = 4); // "40000"
    println!("{x:0>width$}", x = 5, width = 10); // "0000000005"
    println!("{x:0>width$}", x = 1.2345, width = 10); // "00001.2345"

    // ({integer}, {integer}) doesn’t implement std::fmt::Display
    // the trait std::fmt::Display is not implemented for ({integer}, {integer})
    // in format strings you may be able to use {:?} (or {:#?} for pretty-print) instead (rustc E0277)
    println!("{:?}", (1, 2)); // (1, 2)
    println!("{:#?}", (3, 4));
    // (
    //     3,
    //     4,
    // )
}

#[test]
fn debug() {
    #[allow(dead_code)]
    // field 0 is never read
    // consider removing this field
    // CosmosRocket has a derived impl for the trait Debug, but this is intentionally ignored during dead code analysis
    // #[warn(dead_code)] (part of #[warn(unused)]) on by default (rustc dead_code)
    #[derive(Debug)]
    // CosmosRocket doesn’t implement Debug
    // the trait Debug is not implemented for CosmosRocket
    // add #[derive(Debug)] to CosmosRocket or manually impl Debug for CosmosRocket (rustc E0277)
    struct CosmosRocket(i32);

    // CosmosRocket doesn’t implement std::fmt::Display
    // in format strings you may be able to use {:?} (or {:#?} for pretty-print) instead (rustc E0277)
    println!("The name of Cosmos Rocket is {:?}", CosmosRocket(150)); // The name of Cosmos Rocket is CosmosRocket(150)

    #[allow(dead_code)]
    #[derive(Debug)]
    struct BlackholeRocket {
        speed: f64,
        location_x: f64,
        location_y: f64,
        location_z: f64,
    }
    impl BlackholeRocket {
        fn new(speed: f64, location_x: f64, location_y: f64, location_z: f64) -> BlackholeRocket {
            BlackholeRocket {
                speed,
                location_x,
                location_y,
                location_z,
            }
        }
    }
    println!(
        "The name of Cosmos Rocket is {:#?}",
        BlackholeRocket::new(0f64, 1f64, 1f64, 1f64)
    );
}
