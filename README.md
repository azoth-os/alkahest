# :alembic: *Alkahest* Hypervisor Type-1

**Alkahest** est un hyperviseur WebAssembly ([Wasm][WASM]) de **Type-1** à
isolation logicielle (**SFI** - Software Fault Isolation), conçu exclusivement pour le micro-kernel **Athanor** au sein de l'écosystème **Azoth**.

> [!WARNING]
>
> Ce répertoire est en cours de développement actif 🛠️ (Pre-alpha) ⏳. Les APIs internes sont sujettes à des modifications fréquentes 🔄.

[WASM]: https://webassembly.org/

## 🎯 Objectifs

1. **Zero-Context-Switch :** Exécution de tous les modules dans un **Single Address Space (SAS)**. Les appels système ne sont plus des interruptions (`syscall`), mais des appels de fonctions directs.
2. **Sécurité par la Preuve :** Isolation garantie par la validation sémantique du bytecode et l'insertion de gardes logicielles au moment de la compilation AOT.
3. **Performance "Bare-Metal" :** Utilisation de la compilation *Ahead-of-Time* pour atteindre une vitesse d'exécution proche du C/Rust natif.
4. **Résilience (Micro-Recovery) :** Capacité de redémarrer un module crashé en quelques microsecondes en réinitialisant simplement sa session.

AOT

USERLAND -> ALKAHEST -> ATHANOR -> HARDWARD

## License

