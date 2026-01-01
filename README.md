# :alembic: *Alkahest* Hypervisor Type-1

**Alkahest** est un hyperviseur WebAssembly ([Wasm][WASM]) de **Type-1** à
isolation logicielle (**SFI** - Software Fault Isolation), conçu exclusivement pour le micro-kernel **Athanor** au sein de l'écosystème **Azoth**.

S'inspirant de la philosophie "Compiler-as-a-library" de `rustc`,
Alkahest transforme le bytecode Wasm en code machine natif sécurisé, éliminant le compromis historique entre performance brute et sécurisé mémoire.

> [!WARNING]
>
> Ce répertoire est en cours de développement actif 🛠️ (Pre-alpha) ⏳. Les APIs internes sont sujettes à des modifications fréquentes 🔄.

[WASM]: https://webassembly.org/

## 🎯 Objectifs

1. **Zero-Context-Switch :** Exécution de tous les modules dans un **Single Address Space (SAS)**. Les appels système ne sont plus des interruptions (`syscall`), mais des appels de fonctions directs.
2. **Sécurité par la Preuve :** Isolation garantie par la validation sémantique du bytecode et l'insertion de gardes logicielles au moment de la compilation AOT.
3. **Performance "Bare-Metal" :** Utilisation de la compilation *Ahead-of-Time* pour atteindre une vitesse d'exécution proche du C/Rust natif.
4. **Résilience (Micro-Recovery) :** Capacité de redémarrer un module crashé en quelques microsecondes en réinitialisant simplement sa session.

## 🏗️ Architecture (Inspirée de `rustc`)

Alkahest est découpé en crates spécialisées pour garantir la modularité et faciliter les tests en environnement `std` tout en restant `no_std` au runtime.

### 📦 Les Composants du Workspace

* **`alkahest-utils`** : Fondations du système. Définit les types d'adresses pour le SAS et les structures de données de base.
* **`alkahest-diagnostics`** : Système riche de rapports d'erreurs et de logs. Fournit des messages d'erreurs structurés avec codes uniques (ex: `ALKA_001`).
* **`alkahest-session`** : Chef d'orchestre du cycle de vie des modules. Gère la machine à états : `Validating` ➔ `Compiling` ➔ `Linking` ➔ `Ready`.
* **`alkahest-parse`** : Décodeur et validateur de bytecode Wasm (basé sur `wasmparser`). Transforme le binaire en une représentation exploitable.
* **`alkahest-link`** : Le moteur de liaison. Il résout les symboles et injecte les fonctions système (VTable) du noyau Athanor dans les modules.
* **`alkahest-codegen`** : (À venir) Backend AOT transformant l'IR en code machine optimisé pour l'architecture cible.

## 🛡️ Isolation SFI (Software Fault Isolation)

Contrairement aux hyperviseurs classiques (Xen, KVM), Alkahest n'utilise pas la pagination matérielle (MMU) pour séparer les processus. L'isolation est maintenue par :

1.  **Vérification de Bornes :** Chaque accès mémoire est vérifié par le compilateur.
2.  **Shadow Stacks :** Séparation de la pile de données et de la pile de contrôle (adresses de retour).
3.  **Capabilities :** Un module ne peut appeler que les fonctions système que le `Linker` a explicitement liées à sa session.

## 🛠️ Développement

Le projet est configuré pour être compilé sans la bibliothèque standard, mais supporte les tests unitaires via `std`.


