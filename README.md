![littlewood fractal](https://github.com/hrrycx/littlewood-fractal/blob/master/mediumquality.png?raw=true)

# littlewood fractal

- argand diagram plot of all roots of the first 2^25 polynomials with coefficients of 1 and -1 (around 1.5 billion roots)
- colour and lightness decided by density of roots

## running
first run the python code to obtain roots for the polynomials, then use this text file with the rust code to get the fractal. run with `cargo run --release` for best performance, and consider storing the number of roots in each pixel in a new text file (this will be faster than running through the entire roots file for changes to colouring etc).
