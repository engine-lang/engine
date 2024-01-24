# بسم الله الرحمن الرحيم
اللَّهُمَّ انْفَعْنِي بِمَا عَلَّمْتَنِي، وَعَلِّمْنِي مَا يَنْفَعُنِي، وَزِدْنِي عِلْمًا


# Engine
Engine Programming Language - The Next Generation Of Programming Languages


# Introduction
Engine is a new programming language which it's compiler is based on Rust programming language. This has many benefits:

* Rust is fast. with memory safty guaranties
* No garbash collection model.

## Language Features
* It is based on rust, so it is very fast, and memory safe.
* No garbash collection model.
* It actually can be used as
    * Compiler
    * Interpreter
    * Byte Code Generator (That compile your code into byte code file)
    * Byte code executer (Interpreter that execute your code)



# Quick Start

## ## Installing
First clone the repo
```bash
git clone git@github.com:engine-lang/engine.git
```

Then create engine file
```bash
touch test.en
```
and put this code
```engine
string variable_1 = "hello"
print(variable_1)

string variable_2 = ", world!!!!"
print(variable_2)
```


## ## Run engine as Interpreter
You can run engine as an interpreter on the file by just running
```bash
cargo run "test.en"
```

it should print
```bash
hello, world!!!!
```


## ## Run engine as a Compiler
You can run engine as a compiler which will generate an executable.

```bash
cargo run "test.en" -e
```

it will generate an executable file called `test`, and to run it

```bash
./test
```

it should print
```bash
hello, world!!!!
```


## Generate byte Code
You can generate a byte code like this
```bash
cargo run "test.en" -b
```

it will generate a file called `test.en.byte` in this format
```
0:EngineByteCode:v0.1.0
1:Assign:string:"temp_1":"hello"
2:Assign:string:"variable_variable_1":""
3:Convert:string:"variable_variable_1":"temp_1"
4:Print:"variable_variable_1"
5:Assign:string:"temp_2":", world!!!!"
6:Assign:string:"variable_variable_2":""
7:Convert:string:"variable_variable_2":"temp_2"
8:Print:"variable_variable_2"
9:End:
```

you can then use it to your specific cases, or you can run it using engine vm


## Run engine as VM(Virtual Machine)
To execute the engine byte code file, you can run
```bash
cargo run "test.en.byte" --vm
```
it will interprete the file


# Docs
For the full documentation on how to use the language and it's syntax please look at [**Engine Language Docs**](https://github.com/engine-lang/docs)


# How to contribute
To contribute please read [**CONTRIBUTING.md**](./CONTRIBUTING.md).


# Channels
* DM at `abdoaslam000@gmail.com`
