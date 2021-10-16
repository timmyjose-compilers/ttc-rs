This is the grammar for the Teeny Tiny language:

```
  program ::= { statement }  
  statement ::= "PRINT" (expression | string) NL
              | "IF" comparison "THEN" NL { statement } "ENDIF" NL
              | "WHILE" comparison "REPEAT" NL { statement } "ENDWHILE" NL
              | "LABEL" ident NL
              | "GOTO" ident NL
              | "LET" ident "=" expression NL
              | "INPUT" ident NL
  comparison ::= expression ( ("==" | "!=" | "<" | "<=" | ">" | ">=") expression)+
  expression ::= term { ("-" + "+") term }
  term ::= unary { ("*" | "/") unary }
  unary ::= ["+" | "-"] primary
  primary ::= number | ident
  NL ::= "\n"+

```