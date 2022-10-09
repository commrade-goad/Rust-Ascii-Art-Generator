use image::{GenericImageView};
use std::env;
use std::process;

fn get_str_ascii(intent :u8)-> &'static str{
    let index = intent/32;
    let ascii = [" ",".",",","-","~","+","=","@"];
    return ascii[index as usize];
}

fn get_image(dir: &str, scale: u32){
    let img = image::open(dir).unwrap_or_else(|_error| {
        let mut _error = 1;
        println!("Failed to load the image!");
        print_help();
        process::exit(_error);
    });
    println!("{:?}", img.dimensions());
    let (width,height) = img.dimensions();
    for y in 0..height{
        for x in 0..width{
            if y % (scale * 2) == 0 && x % scale ==0{
                let pix = img.get_pixel(x,y);
                let mut intent = pix[0]/3 + pix[1]/3 + pix[2]/3;
                if pix[3] ==0{
                    intent = 0;
                }
                print!("{}",get_str_ascii(intent));
            } 
        }
        if y%(scale*2)==0{
            println!("");
        }
    }
}

fn print_help () {
    println!("Usage: ascii-art [path to file] [scale]");
    println!("Example: ascii-art ~/image.png 4");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let help_args= vec!["-h".to_string(), "--help".to_string()];
    if help_args.iter().any(|e| args.contains(e)) {
        print_help();
        process::exit(0);
    }
    
    if args.len() < 3 {
        println!("Not enought arguments!");
        print_help();
        process::exit(1);
    }
    let convert_scale: u32 = args[2].parse().unwrap();
    get_image(&args[1], convert_scale);
}
