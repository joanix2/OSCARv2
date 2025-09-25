# OSCARv2 - Cellular Automaton Simulation

OSCARv2 est une réimplémentation en Rust du système de simulation d'automates cellulaires OSCAR, avec un système d'affichage graphique intégré.

## Fonctionnalités

- **Parser DSL** : Lecture et analyse de fichiers de configuration au format OSCAR
- **Moteur de simulation** : Exécution de simulations d'automates cellulaires avec agents, champs et règles
- **Affichage graphique** : Interface graphique en temps réel avec la bibliothèque minifb
- **Mode console** : Exécution en mode texte pour les tests et le debugging
- **Interface en ligne de commande** : Configuration flexible via arguments

## Installation et compilation

### Prérequis

- Rust 1.70+ (https://rustup.rs/)
- Système Linux/Windows/macOS avec support graphique

### Compilation

```bash
# Clone du projet
git clone <repository-url>
cd OSCARv2

# Compilation en mode debug
cargo build

# Compilation optimisée (recommandé pour l'exécution)
cargo build --release
```

## Utilisation

### Ligne de commande

```bash
# Affichage de l'aide
./target/release/OSCARv2 --help

# Exécution avec affichage graphique (mode par défaut)
./target/release/OSCARv2

# Spécifier un fichier de configuration
./target/release/OSCARv2 --config mon_niveau.txt

# Mode console uniquement
./target/release/OSCARv2 --console-only

# Personnaliser la vitesse de simulation (en millisecondes)
./target/release/OSCARv2 --tick-time 200

# Personnaliser la taille de la fenêtre
./target/release/OSCARv2 --max-width 1600 --max-height 900
```

### Contrôles en mode graphique

| Touche     | Action                                 |
| ---------- | -------------------------------------- |
| **ESPACE** | Pause/Resume de la simulation          |
| **R**      | Reset de la simulation (à implémenter) |
| **ESC**    | Quitter l'application                  |

## Architecture du système

### Modules principaux

- **`dsl/`** : Parser et AST pour le langage DSL OSCAR
- **`model/`** : Structures de données (World, Agent, Position, etc.)
- **`engine/`** : Moteur de simulation (Engine, Field, Rules, Sensor)
- **`display/`** : Système d'affichage graphique
- **`utils/`** : Utilitaires (logging, couleurs)

### Système d'affichage

Le système d'affichage est basé sur **minifb** et fournit :

#### `DisplaySystem`

- Gestion de la fenêtre graphique
- Rendu des cellules et agents
- Conversion de couleurs (hex/nommées → RGB)
- Dessin optimisé par buffer de pixels

#### `SimulationRunner`

- Boucle principale de simulation
- Intégration engine + affichage
- Gestion des entrées utilisateur
- Contrôle du timing (tick rate)

### Inspiration du code Python

Le système d'affichage s'inspire directement des classes Python fournies :

**Équivalence des fonctionnalités :**

| Python `Win_Canvas` | Rust `DisplaySystem`                   |
| ------------------- | -------------------------------------- |
| `__init__()`        | `new()`                                |
| `refresh()`         | `refresh()`                            |
| `tick()`            | Intégré dans `SimulationRunner::run()` |
| Tkinter Canvas      | minifb Window + Buffer                 |

**Fonctionnalités reprises :**

- ✅ Calcul automatique de la taille des blocs (`BlocSize`)
- ✅ Affichage des agents avec leurs couleurs
- ✅ Support des traces (cercles rouges)
- ✅ Rafraîchissement en temps réel
- ✅ Contrôle du tick rate
- ✅ Gestion de la fenêtre redimensionnable

## Format DSL

Le système utilise le même format DSL que l'OSCAR original :

```
world 64 64 white

mineral rock gray
var solidity 100 0

animal predator red
var energy 50 -1
status energy < 0 dead
birth energy > 80 predator

agent rock (5,5) (10,10) (15,15)
agent predator (32,32)
```

## Exemples d'utilisation

### Simulation basique

```bash
# Lance une simulation 64x64 avec affichage graphique
./target/release/OSCARv2 --config tests/levels/level_0.txt
```

### Test de performance

```bash
# Mode console pour mesurer les performances
./target/release/OSCARv2 --console-only --tick-time 10
```

### Simulation ralentie pour observation

```bash
# Simulation plus lente pour observer les détails
./target/release/OSCARv2 --tick-time 500
```

## Développement

### Ajout de nouvelles fonctionnalités

1. **Nouveaux types d'agents** : Modifier `model/agent.rs` et `dsl/ast.rs`
2. **Nouvelles règles** : Étendre `engine/rules.rs`
3. **Nouveaux champs** : Modifier `engine/field.rs`
4. **Interface graphique** : Étendre `display.rs`

### Debug et tests

```bash
# Tests unitaires
cargo test

# Vérification du code
cargo check

# Mode verbose pour debug
RUST_LOG=debug ./target/release/OSCARv2 --console-only
```

## Comparaison Python vs Rust

| Aspect          | Python OSCAR  | Rust OSCARv2        |
| --------------- | ------------- | ------------------- |
| **Performance** | ~100 FPS      | ~1000+ FPS          |
| **Mémoire**     | ~50MB         | ~5MB                |
| **Startup**     | ~2s           | ~0.1s               |
| **Compilation** | Interprété    | Natif optimisé      |
| **Dépendances** | ezTK, tkinter | minifb seulement    |
| **Portabilité** | Python requis | Exécutable autonome |

## Limitations actuelles

- [ ] Reset de simulation non implémenté
- [ ] Parsing DSL incomplet (sensors, fields complexes)
- [ ] Pas de sauvegarde/chargement d'états
- [ ] Interface utilisateur basique (pas de menus)

## Roadmap

- [ ] Parser DSL complet
- [ ] Interface utilisateur avancée
- [ ] Export de vidéos/images
- [ ] Éditeur de niveaux intégré
- [ ] Support WebAssembly pour le web
- [ ] API REST pour contrôle externe

## Contribution

1. Fork le projet
2. Créer une branche feature (`git checkout -b feature/new-feature`)
3. Commit (`git commit -am 'Add new feature'`)
4. Push (`git push origin feature/new-feature`)
5. Créer une Pull Request

## Licence

Ce projet est sous licence MIT. Voir le fichier `LICENSE` pour plus de détails.
