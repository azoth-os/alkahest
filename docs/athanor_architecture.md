# 🌌 Azoth OS : Architecture Athanor & Alkahest

Ce document détaille la conception technique de l'hyperviseur **Alkahest** et son intégration avec le micro-noyau **Athanor**.

---

## 🏛️ Concept Fondamental : SFI & SAS
Azoth OS utilise l'**Isolation par Faute Logicielle (SFI)** au sein d'un **Espace d'Adressage Unique (SAS)**. Contrairement aux OS traditionnels, nous n'utilisons pas la MMU (pagination) pour séparer les processus. 

- **Athanor (Le Micro-noyau) :** Gère les ressources brutes (RAM, CPU, Interruptions).
- **Alkahest (L'Hyperviseur) :** Garantit la sécurité en compilant le bytecode WebAssembly en code machine natif injecté de "gardes" (sandboxing).

---

## 🛠️ Structure du Workspace (Crates)

Le projet est découpé en crates spécialisées pour assurer une séparation stricte des responsabilités :

| Crate | Rôle |
| :--- | :--- |
| `alkahest` | **La Façade.** Point d'entrée unique pour Athanor. Orchestre les autres crates. |
| `alkahest-core` | **La Fondation.** Définit le trait `Crucible` (contrat noyau) et les types d'adresses SAS. |
| `alkahest-modules` | **Le Registre.** Définit la structure d'un module "vivant" (identifiants, limites mémoire, état). |
| `alkahest-session` | **L'Usine.** Pipeline de chargement temporaire (Validation -> Linking -> Compilation). |
| `alkahest-macros` | **L'Automatisation.** Macros procédurales (`#[driver]`, `#[app]`) pour la déclaration des modules. |
| `alkahest-diagnostics` | **Le Moniteur.** Système structuré de logs et de rapports d'erreurs pour le noyau. |

---

## 🔄 Flux d'exécution type

### 1. Initialisation (Handshake)
Athanor initialise `Alkahest` en lui fournissant une implémentation du trait **`Crucible`**. Ce trait permet à l'hyperviseur de demander de la mémoire brute et de rapporter des erreurs sans connaître les détails internes du noyau.

### 2. Chargement d'un Module
Lorsqu'Athanor veut lancer un module (ex: `init.wasm`) :
1. Une **`Session`** est ouverte.
2. Le binaire est validé (sécurité des types et de la pile).
3. Les dépendances (syscalls) sont résolues via une **VTable** fournie par le noyau.
4. Le code machine est généré. Les instructions d'accès mémoire sont "masquées" pour rester dans les bornes du module.

### 3. Exécution
Une fois compilé, le module est enregistré dans **`alkahest-modules`**. Athanor reçoit le point d'entrée et peut l'exécuter comme une simple fonction. Le passage du noyau au module se fait sans changement de contexte matériel (Zero-Context Switch).

---

## 🔒 Le Contrat Crucible (Kernel Interface)

Pour fonctionner, Alkahest exige que le noyau expose au minimum :
- **Allocation de pages :** Pour stocker le code et la mémoire linéaire des modules.
- **Gestion W^X :** Pouvoir marquer une page comme exécutable après la compilation.
- **Sink de Diagnostic :** Un canal pour envoyer les logs (Série, Framebuffer, etc.).

---
