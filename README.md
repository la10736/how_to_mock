# Rust... How To Mock

Esempio di come costruire un mock/fake in rust con i seguenti vincoli:

1. Il mock non e' mutabile
2. Il mock viene mosso nella classe da testare e non si usa la ref

## TODO

1. Esempio con thread e channel (usare `Arc`?)
2. La classe da testare contiene un adapter da chiamare in un `spawn` del thread