```bash
$ memorus add text @tag !
```

# parser.rs 
```PseudoRust
"memorus add text @tag @tag_2 !!!"

tokens = [memorus, add,text,tag,tag,!!!];

match tokens.get(1){
  "add" => add
  etc.
}
```

```Rust
struct Task{
    text : &str,
    tags : Vec<&str>,
    .timestamp : string,
    importance : [0..3] (u8),
    status : bool,
};
```
# Text format

[status] .timestamp
text : " "
imptnce : [0..3]


# config.json

color

etc.
