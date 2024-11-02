# CRC32 Calculator 

  
## Description
Small rust library package for calculating CRC32 checksums
  
## How does it work 
The library calculates CRC32 checksum value based on the input payload.  
Users can precisely specify the calculation method and boundy parameters (in work).
Currently contains a lot of logging points as the final product is still in the making.
  
## Prerequisites for usage
- git
- rust environment
  
## How to build the library
- checkout the source from remote repository
- go to the root folder of the checked-out sources
- for building a debug version type in cmd: cargo build --debug
- for building a release version type in cmd: cargo build --release
   
## Usage 
The library interface is compact and consist of one function:  
```sh
fn compute_crc32(input: &Vec<u8>, generator: &PolynomialType ) -> Option<u32>
```  
The application using the library CRC32 calculator just has to provide the input payload  
and define the polynomial which is to be used during the calculation.
  
## Running example code
A simple example application has been prepared to display. How one can use the library.  
The example can be build and run with the following cargo command:  
```sh
cargo run --release --example calc_crc32
```
