This crate simplifies the initialization of const variable by using `addr!()` macro. It expects a string literal. See following example:

```
use address_literal::addr;

const BOT: H160 = addr!("0x0000000279be4c78d026d348651e26f3fcc8cf90");
```

