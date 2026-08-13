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
    text : string,
    tags : Vec<string>,
    .timestamp : string,
    importance : [0..3] (usize),
    status : bool,
};
```
# Text format

[status] 
timestamp  : 
text : " "
tags : 
imptnce : [0..3]
end.



# config.json

color

etc.
