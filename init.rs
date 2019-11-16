//INPUT = SET OF OBJECTS
//[{
//	language:"RUST",
//	init:"fn main() {println!(\"Hello World!\");"}
//	apps /// attention ///
//}];

//SEQUENTUALLY HASH OBJ AS TRAVERSE 
	//RECURSION
	
//REFORM BOUDING RADIUS CALCULATION FOR CONSENSUS CALCULATION (PARAMS ARE CONTAINED IN OBJ)

//CONENCTION INSIGHT COMING

//HERE


//.CRE8 
//{
//	language:"RUST",
//	init:"fn main() {println!(\"Hello World!\");"}
//	apps /// attention ///
//}

//THINK ABOUT CONNECTIONS AND TREE REDUCTION (WHAT IS THE LANG - LANG CONNECTIONS .. IT IS THE COMPILER >:P)
//FLASH COURSE IN TREE REDUCTION  

//THIS READS THE JSON FILE IN RUST.. 
//NEEDS TO BE WRAPPED.. 
//& DESERIALIZE .CRE8
//SOME INIT FUNCTIONS TO STRI WRAPPER AND APPLY RUNTIMEZ

//RECURSIVE.. LOL
//COMPILER :) 

//CRE8 PACKAGE MANAGER DOES THIS INTEROP ~ USES IMPORT 
	//ACTS AS CROSS LOANGUAGE INTEROP..
	//BY FLATTENING TO CRETE / RUST NOW FLOOR
		//CAN COMPILE TO STD .ASM //AND ASM.JS EQUIVILANT FOR TRUE INTEROP

//.CRE8 specifications, language of lanugages
	//-->rs converter.. 
	//-->js converter..
	//-->py converter..
		//...
		//specical charaters and logic
//convert .CRE8 to .rs // if language is rust;
//.CRE8 is a suberset grammer

#![feature(rustc_private)]

extern crate rustc;
extern crate rustc_interface;
extern crate rustc_driver;

extern crate serde;
extern crate serde_json;


//phew, that requires a lower level interface than the one I showed.
// It's certainly possible, but you need to create the AST (the HIR) yourself. I suggest you look at the librustc* and see how the rustc binary is created.
//https://blog.rust-lang.org/images/2016-04-MIR/flow.svg
//https://emscripten.org/index.html
//-L $HOME/.multirust/toolchains/nightly/lib/rustlib/x86_64-unknown-linux-gnu/lib
//rustc package produces c binary
//wasm-bindgen facilitates high-level interactions between Rust and JavaScript. It allows one to import JavaScript things into Rust and export Rust things to JavaScript.
//wasm-bindgen-futures is a bridge connecting JavaSript Promises and Rust Futures. It can convert in both directions and is useful when working with asynchronous tasks in Rust, and allows interacting with DOM events and I/O operations.
//js-sys Raw wasm-bindgen imports for all the JavaScript global types and methods, such as Object, Function, eval, etc. These APIs are portable across all standard ECMAScript environments, not just the Web, such as Node.js.


//CREATE --> RUST --> (C) 
fn main() {

	//NEED TO BE AS CODE FORMED
	//FIRST CONNECT TO NODE -->> NODE DISCOVERY ..

	//GET AVAILABLE PACKAGES






    let the_file = r#"{

    	"language":"RUST",
    	"init":"fn main() {println!(\"HI\")}"

    }"#;

    let json: serde_json::Value = serde_json::from_str(the_file).expect("JSON was not well-formatted");

    println!("{}", json["init"]);

	//let args: Vec<_> = std::env::args().collect();
	//let args: Vec<_> = json["init"];
	//rustc_driver::run_compiler(&args, &mut "", &mut "", &mut "",);
	// println!("{:#?}", args);

	//CONFIG TYPE..
	//let test: rustc::session::config::Input = "";

	//PASS TO COMPILER
    //rustc_interface::interface::run_compiler(test);





}

