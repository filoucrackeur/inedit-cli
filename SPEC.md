# ini-editor - Spécification du projet

## 1. Overview

- **Nom**: ini-editor
- **Type**: Application CLI avec interface TUI interactive
- **Fonction**: Éditeur interactif pour fichiers de configuration INI/.conf avec navigation au clavier
- **Utilisateurs**: Administrateurs système, développeurs

## 2. Fonctionnalités

### 2.1 Opérations CRUD
- **Ajouter** une variable (clé=valeur) dans une section existante ou nouvelle
- **Modifier** la clé ou la valeur d'une variable existante
- **Supprimer** une variable
- **Commenter/Décommenter** une variable (# au début)

### 2.2 Gestion des sections
- Ajouter de nouvelles sections `[section]`
- Modifier le nom d'une section
- Supprimer une section (avec toutes ses variables)

### 2.3 Navigation
- Navigation par `Tab` entre les zones prédéfinies:
  - Liste des sections (panneau gauche)
  - Liste des variables de la section sélectionnée (panneau central)
  - Éditeur de valeur (zone de saisie en bas)
- Flèches haut/bas pour déplacer dans les listes
- Entrée pour sélectionner/éditer

### 2.4 Commandes clavier
- `Tab` / `Shift+Tab`: Navigation entre zones
- `Flèches`: Navigation dans les listes
- `Entrée`: Éditer la valeur sélectionnée / valider
- `Échap`: Annuler l'édition
- `a`: Ajouter (section ou variable selon contexte)
- `m`: Modifier l'élément sélectionné
- `d`: Supprimer l'élément sélectionné
- `c`: Commenter/Décommenter
- `Ctrl+s`: Sauvegarder et quitter
- `Ctrl+q`: Quitter sans sauvegarder
- `?`: Afficher l'aide

## 3. UI/UX Structure

```
┌─────────────────────────────────────────────────────────┐
│  ini-editor - monfichier.ini                    [?]    │
├─────────────┬───────────────────────────────────────────┤
│ SECTIONS    │  VARIABLES                                │
│             │                                           │
│ [default] ● │  server = localhost          [c] [m] [d]│
│ [database]  │  port = 5432               [c] [m] [d]  │
│ [logging]   │  debug = true               [c] [m] [d]  │
│             │                                           │
├─────────────┴───────────────────────────────────────────┤
│ ÉDITEUR: Valeur de 'server'                             │
│ > localhost________________________________            │
├─────────────────────────────────────────────────────────┤
│ Tab:Navigation | a:Ajouter m:Modifier d:Supprimer c:    │
│      Commenter | Ctrl+S:Sauvegarder Ctrl+Q:Quitter       │
└─────────────────────────────────────────────────────────┘
```

## 4. Architecture

### 4.1 Dépendances (Cargo.toml)
- `ratatui`: Bibliothèque TUI principale
- `crossterm`: Gestion des entrées terminal
- `ini`: Parser INI
- `clap`: Parseur d'arguments CLI

### 4.2 Modules
- `main.rs`: Point d'entrée, gestion CLI
- `parser.rs`: Lecture/écriture fichiers INI
- `ui.rs`: Interface utilisateur TUI
- `state.rs`: État de l'application

## 5. Critères de succès

- L'application démarre et affiche le fichier INI
- La navigation Tab fonctionne entre les 3 zones
- Les opérations CRUD sont fonctionnelles
- Les commentaires sont gérés
- Sauvegarde correcte du fichier modifié