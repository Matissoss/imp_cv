# Contributing Guide

This is short guide to add more units

---

## File structure

To make the files as clean as possible i used this tree:

``` 
src/
    main.rs
    length/
        imperial/
            meters_to_foot.rs
            ..
        normal/
            foot_to_meters.rs
            ..
    weigth/
        imperial/
            kilograms_to_pounds.rs
            ..
        normal/
            pounds_to_kilograms.rs
            ..
    temperature/
        farenheit.rs
        celcius.rs
```

If you want to add `Imperial Unit -> Normal Unit` add it in normal subdirectory

If you want to add `Normal Unit -> Imperial Unit` add it in imperial subdirectory


## Include your Units in main.rs

To add your unit in `main.rs`, think about flag you want to use 

for example:

`-i -yf unit`

`i` means that `main.rs` will search in imperial subdirectories and

`n` means that `main.rs` will search in normal subdirectories;

`yf` is where your flag should be;

`unit` is unit you want to convert;

Next, create your file in appropiate directory with ending `.rs`

---

To finally include your file in `main.rs` search for this fragment:

```
mod length{
    pub mod imperial{
    ..
    }
    pub mod normal{
    ..
    }
}
```

(if you want to use weigth or temperature search for `mod weigth {..}` or `mod temperature{..}`)

Include your file in these submod's using `pub mod yf;`

`yf` is your file name;

---

Add the `yf` in `use` chain

---

Then search for `if` chain in `fn main() {..}` and add your flag.

---

Explaination of args

`args[0]` shouldn't be used/won't be needed

`args[1]` contains info if command refers to imperial or normal

`args[2]` contains your/other flags

`args[3]` contains value user want to convert

## Include your Units in logs.rs

`logs.rs` is responsible for showing help message after using `cargo run -- --help`
Here you should add your unit to make sure that final user sees your unit ;
Like this:

```
Normal to Imperial:
..
-n -yf x -> converts x to y
..
```
`x` is unit you want to convert and

`y` is unit you want to convert to

`yf` is your flag


### Small detail

I'm not a english native, so sorry for my bad english (just in case)
