```bash
$ memorus add text @tag ^
```

# task.rs/create_task done.
  ```PseudoRust
  "memorus add text @tag @tag_2 ^^^"
  
  tokens = [text,tag,tag,^^^];
  
  match tokens.get(1){
    tag=>tag.push(),
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
# task.rs/token functions

for EVERY token(text,tags, etc.) 
must be selected function-coder to text format

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
