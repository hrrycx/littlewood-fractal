mod hsl;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use image::{RgbImage, Rgb, imageops};
use image::imageops::FilterType;
use image::ImageFormat::Tiff;
use image::ImageFormat::Png;
use image::Pixel;
const HEIGHT : u32 = 1080 * 10;
const WIDTH : u32 = 1920 * 10;
fn main() -> io::Result<()> {
    let file = File::open("C:/Users/itsmr/Desktop/gen output folder/roots.txt")?;
    let reader = BufReader::new(file);
    let mut count = vec![vec![0_i32; WIDTH as usize]; HEIGHT as usize];

    for line in reader.lines() {
        let (re, im) = get_re_im(line?);
        count[yp(im) as usize][xp(re) as usize] += 1;
    }
    let mut highest : i32 = 0;

    for row in &count{
        for item in row{
            if highest < *item{
                highest = *item;
            }
        }
    }
    let mut img = RgbImage::new(WIDTH, HEIGHT);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let value_at_pixel = (count[y as usize][x as usize] as f64).log2() / (highest as f64).log2();
            img.put_pixel(x.try_into().unwrap(), y.try_into().unwrap(), *Rgb::from_slice(&hsl::hsl_to_rgba((value_at_pixel / 2.) - 0.08, 1., value_at_pixel * 1.5)));
        }
    }
    let img2 = imageops::resize(&mut img, 1920, 1080, FilterType::Gaussian);
    let _ = img.save_with_format("highquality.tiff", Tiff);
    let _ = img2.save_with_format("mediumquality.png",Png);
    Ok(())
}
fn yp(y : f64)-> i32{
    return (((((y) / (1.5)) + 1.) / 2.) * HEIGHT as f64) as i32;
}
fn xp(x : f64)-> i32{
    return (((((x) / (8./3.)) + 1.) / 2.) * WIDTH as f64) as i32;
}
fn get_re_im(line: String) -> (f64, f64){
    if !line.contains("j"){
        return (line.parse().unwrap(), 0.) 
    }
    let pos: usize;
    if line.contains("+"){
        pos = line.find("+").unwrap();
    }
    else if !line[1..].contains("-"){
        return (0., line[0..line.len()-1].parse().unwrap())
    }
    else{
        if line.contains("e"){
            let pos1 = line[2..line.len()].find("-").unwrap()+2;
            let pos2 = line.find("e").unwrap();
            if pos2 < pos1{
                pos = pos2 + 4;
            }
            else{
                pos = pos1;
            }
        }
        else{
            pos = line[2..line.len()].find("-").unwrap()+2;
        }
    }
    let restr = &line[1..pos];
    let re : f64;
    if restr.contains("e"){
        let epos = restr.find("e").unwrap();
        re = restr[0..epos].parse::<f64>().unwrap() * 10_f64.powi(restr[epos+1..epos+4].parse().unwrap());
    }
    else{
        re = restr.parse().unwrap();
    }    
    let imstr: &str;

    imstr = &line[pos..line.find("j").unwrap()];

    let im : f64;
    if imstr.contains("e"){
        let epos = imstr.find("e").unwrap();
        im = imstr[0..epos].parse::<f64>().unwrap() * 10_f64.powi(imstr[epos+1..epos+4].parse().unwrap());
    }
    else {
        im = imstr.parse().unwrap();
    }
    return (re,im)
}