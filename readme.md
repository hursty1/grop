# GROP


This is a learning project for rust, creating a custom version of GREP for use on windows, or linux 


Features 
- search text highlighting
- glob checking (*.txt or *.json)
- case senstive (or insenitive) -i

Example use:
Usage: grop.exe [OPTIONS] <QUERY> <FILE>

Arguments:
  <QUERY>  Query string to search for
  <FILE>   File Name to search

Options:
  -i, --ignore-case  ignore case when searching
  -f                 Print filename
  -r                 recursive directory searching
  -h, --help         Print help
  -V, --version      Print version



  Installations

  mkdir C:/Utils

  cargo build --release

  cp .\target\release\grop.exe c:\utils\grop.exe

  edit path variable and add C:\utilis\ as a folder
  restart windows