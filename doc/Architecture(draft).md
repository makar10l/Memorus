```bash
$ memorus add text @tag !
```

# parser.rs done.
  ```PseudoRust
  "memorus add text @tag @tag_2 !!!"
  
  tokens = [memorus, add,text,tag,tag,!!!];
  
  match tokens.get(1){
    "add" => add
    etc.
  }
  ```

# task.rs done.
  ```Rust
  struct Task{
      text : string,
      tags : Vec<string>,
      .timestamp : string,
      importance : [0..3] (usize),
      status : bool,
  };
  ```
  
# Text format 50/50(only decoder)

  [status] 
  timestamp  : 
  text : " "
  tags : 
  importance : [0..3]
  end.

# config.json 0.

  color
  
  etc.
