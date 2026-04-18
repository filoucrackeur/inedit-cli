use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

pub static CURRENT_LANGUAGE: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new("fr_FR".to_string()));

pub struct Language {
    pub code: &'static str,
    #[allow(dead_code)]
    pub name: &'static str,
    pub native_name: &'static str,
}

pub const AVAILABLE_LANGUAGES: &[Language] = &[
    Language {
        code: "fr_FR",
        name: "French",
        native_name: "Français",
    },
    Language {
        code: "en_US",
        name: "English",
        native_name: "English",
    },
    Language {
        code: "es_ES",
        name: "Spanish",
        native_name: "Español",
    },
    Language {
        code: "de_DE",
        name: "German",
        native_name: "Deutsch",
    },
    Language {
        code: "it_IT",
        name: "Italian",
        native_name: "Italiano",
    },
    Language {
        code: "pt_BR",
        name: "Portuguese (Brazil)",
        native_name: "Português (Brasil)",
    },
    Language {
        code: "pt_PT",
        name: "Portuguese (Portugal)",
        native_name: "Português (Portugal)",
    },
    Language {
        code: "fi_FI",
        name: "Finnish",
        native_name: "Suomi",
    },
    Language {
        code: "sv_SE",
        name: "Swedish",
        native_name: "Svenska",
    },
    Language {
        code: "da_DK",
        name: "Danish",
        native_name: "Dansk",
    },
    Language {
        code: "nb_NO",
        name: "Norwegian (Bokmål)",
        native_name: "Norsk (Bokmål)",
    },
    Language {
        code: "th_TH",
        name: "Thai",
        native_name: "ภาษาไทย",
    },
    Language {
        code: "vi_VN",
        name: "Vietnamese",
        native_name: "Tiếng Việt",
    },
    Language {
        code: "km_KH",
        name: "Khmer (Cambodian)",
        native_name: "ភាសាខ្មែរ",
    },
    Language {
        code: "ja_JP",
        name: "Japanese",
        native_name: "日本語",
    },
    Language {
        code: "zh_CN",
        name: "Chinese (Simplified)",
        native_name: "简体中文",
    },
    Language {
        code: "ru_RU",
        name: "Russian",
        native_name: "Русский",
    },
    Language {
        code: "tr_TR",
        name: "Turkish",
        native_name: "Türkçe",
    },
    Language {
        code: "am_ET",
        name: "Amharic (Ethiopian)",
        native_name: "አማርኛ",
    },
    Language {
        code: "ar_SA",
        name: "Arabic",
        native_name: "العربية",
    },
    Language {
        code: "ko_KR",
        name: "Korean",
        native_name: "한국어",
    },
    Language {
        code: "id_ID",
        name: "Indonesian",
        native_name: "Bahasa Indonesia",
    },
    Language {
        code: "el_GR",
        name: "Greek",
        native_name: "Ελληνικά",
    },
    Language {
        code: "ro_RO",
        name: "Romanian",
        native_name: "Română",
    },
    Language {
        code: "et_EE",
        name: "Estonian",
        native_name: "Eesti",
    },
    Language {
        code: "lv_LV",
        name: "Latvian",
        native_name: "Latviešu",
    },
    Language {
        code: "lt_LT",
        name: "Lithuanian",
        native_name: "Lietuvių",
    },
];

pub type TranslationMap = HashMap<&'static str, &'static str>;

fn make_fr_fr() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "Recherche");
    m.insert("search_active", "Recherche");
    m.insert("section_next", "Zone suivante");
    m.insert("section_prev", "Zone precedente");
    m.insert("move_list", "Deplacer dans les listes");
    m.insert("activate_search", "Activer recherche");
    m.insert("next_prev_result", "Resultat suivant/precedent");
    m.insert("go_result", "Aller au resultat");
    m.insert("quit_search", "Quitter recherche");
    m.insert("edit_value", "Editer la valeur");
    m.insert("rename_key", "Renommer la cle");
    m.insert("cancel_edition", "Annuler edition");
    m.insert("add_section_var", "Ajouter (section/variable)");
    m.insert("delete", "Supprimer");
    m.insert("comment_uncomment", "Commenter/Decommenter");
    m.insert("quit_no_save", "Quitter sans sauvegarder");
    m.insert("save_quit", "Sauvegarder et quitter");
    m.insert("help_quit", "Aide (q pour quitter)");
    m.insert("choose", "Choisir");
    m.insert("confirm_delete_section", "CONFIRMER SUPPRESSION SECTION");
    m.insert("yes", "Oui");
    m.insert("no", "Non");
    m.insert("navigate", "Naviguer");
    m.insert("select", "Selectionner");
    m.insert("validate", "Valider");
    m.insert("close", "Fermer");
    m.insert("new_name", "Nouveau nom");
    m.insert("new_value", "Nouvelle valeur");
    m.insert("rename", "Renommer");
    m.insert("cancel", "Annuler");
    m.insert("next_result", "Suivant");
    m.insert("prev_result", "Precedent");
    m.insert("to_variables", "Variables");
    m.insert("to_sections", "Sections");
    m.insert("edit", "Editer");
    m.insert("add", "Ajouter");
    m.insert("search_label", "Rechercher");
    m.insert("language", "Langue");
    m.insert("select_language", "Selectionner la langue");
    m.insert("language_changed", "Langue changee");
    m.insert("help_title", "Aide - Touches disponibles");
    m.insert("file_saved", "Fichier sauvegarde");
    m.insert("file_save_error", "Erreur lors de la sauvegarde");
    m.insert("saved", "Sauvegarder");
    m.insert("navigation", "Navigation");
    m.insert("inline_edition", "Edition inline");
    m.insert("actions", "Actions");
    m.insert("commands", "Commandes");
    m.insert("sections", "SECTIONS");
    m.insert("variables", "VARIABLES");
    m.insert("editing", "[Editing]");
    m
}

fn make_en_us() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "Search");
    m.insert("search_active", "Search");
    m.insert("section_next", "Next area");
    m.insert("section_prev", "Previous area");
    m.insert("move_list", "Move in lists");
    m.insert("activate_search", "Activate search");
    m.insert("next_prev_result", "Next/previous result");
    m.insert("go_result", "Go to result");
    m.insert("quit_search", "Quit search");
    m.insert("edit_value", "Edit value");
    m.insert("rename_key", "Rename key");
    m.insert("cancel_edition", "Cancel edition");
    m.insert("add_section_var", "Add (section/variable)");
    m.insert("delete", "Delete");
    m.insert("comment_uncomment", "Comment/Uncomment");
    m.insert("quit_no_save", "Quit without saving");
    m.insert("save_quit", "Save and quit");
    m.insert("help_quit", "Help (q to quit)");
    m.insert("choose", "Choose");
    m.insert("confirm_delete_section", "CONFIRM DELETE SECTION");
    m.insert("yes", "Yes");
    m.insert("no", "No");
    m.insert("navigate", "Navigate");
    m.insert("select", "Select");
    m.insert("validate", "Validate");
    m.insert("close", "Close");
    m.insert("new_name", "New name");
    m.insert("new_value", "New value");
    m.insert("rename", "Rename");
    m.insert("cancel", "Cancel");
    m.insert("next_result", "Next");
    m.insert("prev_result", "Previous");
    m.insert("to_variables", "Variables");
    m.insert("to_sections", "Sections");
    m.insert("edit", "Edit");
    m.insert("add", "Add");
    m.insert("search_label", "Search");
    m.insert("language", "Language");
    m.insert("select_language", "Select language");
    m.insert("language_changed", "Language changed");
    m.insert("help_title", "Help - Available keys");
    m.insert("file_saved", "File saved");
    m.insert("file_save_error", "Error saving file");
    m.insert("saved", "Save");
    m.insert("navigation", "Navigation");
    m.insert("inline_edition", "Inline edition");
    m.insert("actions", "Actions");
    m.insert("commands", "Commands");
    m.insert("sections", "SECTIONS");
    m.insert("variables", "VARIABLES");
    m.insert("editing", "[Editing]");
    m
}

fn make_es_es() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "Buscar");
    m.insert("search_active", "Buscar");
    m.insert("section_next", "Siguiente area");
    m.insert("section_prev", "Area anterior");
    m.insert("move_list", "Mover en listas");
    m.insert("activate_search", "Activar busqueda");
    m.insert("next_prev_result", "Resultado siguiente/anterior");
    m.insert("go_result", "Ir al resultado");
    m.insert("quit_search", "Salir de busqueda");
    m.insert("edit_value", "Editar valor");
    m.insert("rename_key", "Renombrar clave");
    m.insert("cancel_edition", "Cancelar edicion");
    m.insert("add_section_var", "Agregar (seccion/variable)");
    m.insert("delete", "Eliminar");
    m.insert("comment_uncomment", "Comentar/Descomentar");
    m.insert("quit_no_save", "Salir sin guardar");
    m.insert("save_quit", "Guardar y salir");
    m.insert("help_quit", "Ayuda (q para salir)");
    m.insert("choose", "Elegir");
    m.insert("confirm_delete_section", "CONFIRMAR ELIMINAR SECCION");
    m.insert("yes", "Si");
    m.insert("no", "No");
    m.insert("navigate", "Navegar");
    m.insert("select", "Seleccionar");
    m.insert("validate", "Validar");
    m.insert("close", "Cerrar");
    m.insert("new_name", "Nuevo nombre");
    m.insert("new_value", "Nuevo valor");
    m.insert("rename", "Renombrar");
    m.insert("cancel", "Cancelar");
    m.insert("next_result", "Siguiente");
    m.insert("prev_result", "Anterior");
    m.insert("to_variables", "Variables");
    m.insert("to_sections", "Secciones");
    m.insert("edit", "Editar");
    m.insert("add", "Agregar");
    m.insert("search_label", "Buscar");
    m.insert("language", "Idioma");
    m.insert("select_language", "Seleccionar idioma");
    m.insert("language_changed", "Idioma cambiado");
    m.insert("help_title", "Ayuda - Teclas disponibles");
    m.insert("file_saved", "Archivo guardado");
    m.insert("file_save_error", "Error al guardar archivo");
    m.insert("saved", "Guardar");
    m.insert("navigation", "Navegacion");
    m.insert("inline_edition", "Edicion en linea");
    m.insert("actions", "Acciones");
    m.insert("commands", "Comandos");
    m.insert("sections", "SECCIONES");
    m.insert("variables", "VARIABLES");
    m.insert("editing", "[Editando]");
    m
}

fn make_de_de() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "Suche");
    m.insert("search_active", "Suche");
    m.insert("section_next", "Nachster Bereich");
    m.insert("section_prev", "Vorheriger Bereich");
    m.insert("move_list", "In Listen bewegen");
    m.insert("activate_search", "Suche aktivieren");
    m.insert("next_prev_result", "Nachstes/Vorheriges Ergebnis");
    m.insert("go_result", "Zum Ergebnis gehen");
    m.insert("quit_search", "Suche beenden");
    m.insert("edit_value", "Wert bearbeiten");
    m.insert("rename_key", "Schussel umbenennen");
    m.insert("cancel_edition", "Bearbeitung abbrechen");
    m.insert("add_section_var", "Hinzufugen (Sektion/Variable)");
    m.insert("delete", "Loschen");
    m.insert("comment_uncomment", "Kommentieren/Dekommentieren");
    m.insert("quit_no_save", "Ohne Speichern beenden");
    m.insert("save_quit", "Speichern und beenden");
    m.insert("help_quit", "Hilfe (q zum Beenden)");
    m.insert("choose", "Wahlen");
    m.insert("confirm_delete_section", "LOSCHEN DER SEKTION BESTATIGEN");
    m.insert("yes", "Ja");
    m.insert("no", "Nein");
    m.insert("navigate", "Navigieren");
    m.insert("select", "Auswahlen");
    m.insert("validate", "Bestatigen");
    m.insert("close", "Schliessen");
    m.insert("new_name", "Neuer Name");
    m.insert("new_value", "Neuer Wert");
    m.insert("rename", "Umbenennen");
    m.insert("cancel", "Abbrechen");
    m.insert("next_result", "Nachstes");
    m.insert("prev_result", "Vorheriges");
    m.insert("to_variables", "Variablen");
    m.insert("to_sections", "Sektionen");
    m.insert("edit", "Bearbeiten");
    m.insert("add", "Hinzufugen");
    m.insert("search_label", "Suchen");
    m.insert("language", "Sprache");
    m.insert("select_language", "Sprache auswahlen");
    m.insert("language_changed", "Sprache geandert");
    m.insert("help_title", "Hilfe - Verfugbare Tasten");
    m.insert("file_saved", "Datei gespeichert");
    m.insert("file_save_error", "Fehler beim Speichern");
    m.insert("saved", "Speichern");
    m.insert("navigation", "Navigation");
    m.insert("inline_edition", "Inline-Bearbeitung");
    m.insert("actions", "Aktionen");
    m.insert("commands", "Befehle");
    m.insert("sections", "SEKTIONEN");
    m.insert("variables", "VARIABLEN");
    m.insert("editing", "[Bearbeiten]");
    m
}

fn make_it_it() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "Ricerca");
    m.insert("search_active", "Ricerca");
    m.insert("section_next", "Prossima area");
    m.insert("section_prev", "Area precedente");
    m.insert("move_list", "Sposta negli elenchi");
    m.insert("activate_search", "Attiva ricerca");
    m.insert("next_prev_result", "Risultato successivo/precedente");
    m.insert("go_result", "Vai al risultato");
    m.insert("quit_search", "Esci dalla ricerca");
    m.insert("edit_value", "Modifica valore");
    m.insert("rename_key", "Rinomina chiave");
    m.insert("cancel_edition", "Annulla modifica");
    m.insert("add_section_var", "Aggiungi (sezione/variabile)");
    m.insert("delete", "Elimina");
    m.insert("comment_uncomment", "Commenta/Scommenta");
    m.insert("quit_no_save", "Esci senza salvare");
    m.insert("save_quit", "Salva ed esci");
    m.insert("help_quit", "Aiuto (q per uscire)");
    m.insert("choose", "Scegli");
    m.insert("confirm_delete_section", "CONFERMA ELIMINAZIONE SEZIONE");
    m.insert("yes", "Si");
    m.insert("no", "No");
    m.insert("navigate", "Naviga");
    m.insert("select", "Seleziona");
    m.insert("validate", "Convalida");
    m.insert("close", "Chiudi");
    m.insert("new_name", "Nuovo nome");
    m.insert("new_value", "Nuovo valore");
    m.insert("rename", "Rinomina");
    m.insert("cancel", "Annulla");
    m.insert("next_result", "Successivo");
    m.insert("prev_result", "Precedente");
    m.insert("to_variables", "Variabili");
    m.insert("to_sections", "Sezioni");
    m.insert("edit", "Modifica");
    m.insert("add", "Aggiungi");
    m.insert("search_label", "Cerca");
    m.insert("language", "Lingua");
    m.insert("select_language", "Seleziona lingua");
    m.insert("language_changed", "Lingua modificata");
    m.insert("help_title", "Aiuto - Tasti disponibili");
    m.insert("file_saved", "File salvato");
    m.insert("file_save_error", "Errore nel salvataggio");
    m.insert("saved", "Salva");
    m.insert("navigation", "Navigazione");
    m.insert("inline_edition", "Modifica in linea");
    m.insert("actions", "Azioni");
    m.insert("commands", "Comandi");
    m.insert("sections", "SEZIONI");
    m.insert("variables", "VARIABILI");
    m.insert("editing", "[Modifica]");
    m
}

fn make_pt_br() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "Pesquisa");
    m.insert("search_active", "Pesquisa");
    m.insert("section_next", "Proxima area");
    m.insert("section_prev", "Area anterior");
    m.insert("move_list", "Mover nas listas");
    m.insert("activate_search", "Ativar pesquisa");
    m.insert("next_prev_result", "Proximo/anterior resultado");
    m.insert("go_result", "Ir para resultado");
    m.insert("quit_search", "Sair da pesquisa");
    m.insert("edit_value", "Editar valor");
    m.insert("rename_key", "Renomear chave");
    m.insert("cancel_edition", "Cancelar edicao");
    m.insert("add_section_var", "Adicionar (secao/variavel)");
    m.insert("delete", "Excluir");
    m.insert("comment_uncomment", "Comentar/Descomentar");
    m.insert("quit_no_save", "Sair sem salvar");
    m.insert("save_quit", "Salvar e sair");
    m.insert("help_quit", "Ajuda (q para sair)");
    m.insert("choose", "Escolher");
    m.insert("confirm_delete_section", "CONFIRMAR EXCLUSAO DA SECAO");
    m.insert("yes", "Sim");
    m.insert("no", "Nao");
    m.insert("navigate", "Navegar");
    m.insert("select", "Selecionar");
    m.insert("validate", "Validar");
    m.insert("close", "Fechar");
    m.insert("new_name", "Novo nome");
    m.insert("new_value", "Novo valor");
    m.insert("rename", "Renomear");
    m.insert("cancel", "Cancelar");
    m.insert("next_result", "Proximo");
    m.insert("prev_result", "Anterior");
    m.insert("to_variables", "Variaveis");
    m.insert("to_sections", "Secoes");
    m.insert("edit", "Editar");
    m.insert("add", "Adicionar");
    m.insert("search_label", "Pesquisar");
    m.insert("language", "Idioma");
    m.insert("select_language", "Selecionar idioma");
    m.insert("language_changed", "Idioma alterado");
    m.insert("help_title", "Ajuda - Teclas disponiveis");
    m.insert("file_saved", "Arquivo salvo");
    m.insert("file_save_error", "Erro ao salvar arquivo");
    m.insert("saved", "Salvar");
    m.insert("navigation", "Navegacao");
    m.insert("inline_edition", "Edicao em linha");
    m.insert("actions", "Acoes");
    m.insert("commands", "Comandos");
    m.insert("sections", "SECOES");
    m.insert("variables", "VARIAVEIS");
    m.insert("editing", "[Editando]");
    m
}

fn make_ja_jp() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "検索");
    m.insert("search_active", "検索");
    m.insert("section_next", "次のエリア");
    m.insert("section_prev", "前のエリア");
    m.insert("move_list", "リストを移動");
    m.insert("activate_search", "検索を有効にする");
    m.insert("next_prev_result", "次の結果/前の結果");
    m.insert("go_result", "結果に移動");
    m.insert("quit_search", "検索を終了");
    m.insert("edit_value", "値を編集");
    m.insert("rename_key", "キーを名前変更");
    m.insert("cancel_edition", "編集をキャンセル");
    m.insert("add_section_var", "追加 (セクション/変数)");
    m.insert("delete", "削除");
    m.insert("comment_uncomment", "コメント化/コメント解除");
    m.insert("quit_no_save", "保存せずに終了");
    m.insert("save_quit", "保存して終了");
    m.insert("help_quit", "ヘルプ (q で終了)");
    m.insert("choose", "選択");
    m.insert("confirm_delete_section", "セクションの削除を確認");
    m.insert("yes", "はい");
    m.insert("no", "いいえ");
    m.insert("navigate", "ナビゲート");
    m.insert("select", "選択");
    m.insert("validate", "確認");
    m.insert("close", "閉じる");
    m.insert("new_name", "新しい名前");
    m.insert("new_value", "新しい値");
    m.insert("rename", "名前変更");
    m.insert("cancel", "キャンセル");
    m.insert("next_result", "次へ");
    m.insert("prev_result", "前へ");
    m.insert("to_variables", "変数");
    m.insert("to_sections", "セクション");
    m.insert("edit", "編集");
    m.insert("add", "追加");
    m.insert("search_label", "検索");
    m.insert("language", "言語");
    m.insert("select_language", "言語を選択");
    m.insert("language_changed", "言語が変更されました");
    m.insert("help_title", "ヘルプ - 利用可能なキー");
    m.insert("file_saved", "ファイル保存完了");
    m.insert("file_save_error", "ファイル保存エラー");
    m.insert("saved", "保存");
    m.insert("navigation", "ナビゲーション");
    m.insert("inline_edition", "インライン編集");
    m.insert("actions", "アクション");
    m.insert("commands", "コマンド");
    m.insert("sections", "セクション");
    m.insert("variables", "変数");
    m.insert("editing", "[編集]");
    m
}

fn make_zh_cn() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "搜索");
    m.insert("search_active", "搜索");
    m.insert("section_next", "下一区域");
    m.insert("section_prev", "上一区域");
    m.insert("move_list", "在列表中移动");
    m.insert("activate_search", "激活搜索");
    m.insert("next_prev_result", "下一个/上一个结果");
    m.insert("go_result", "跳转到结果");
    m.insert("quit_search", "退出搜索");
    m.insert("edit_value", "编辑值");
    m.insert("rename_key", "重命名键");
    m.insert("cancel_edition", "取消编辑");
    m.insert("add_section_var", "添加 (章节/变量)");
    m.insert("delete", "删除");
    m.insert("comment_uncomment", "注释/取消注释");
    m.insert("quit_no_save", "不保存退出");
    m.insert("save_quit", "保存并退出");
    m.insert("help_quit", "帮助 (q 退出)");
    m.insert("choose", "选择");
    m.insert("confirm_delete_section", "确认删除章节");
    m.insert("yes", "是");
    m.insert("no", "否");
    m.insert("navigate", "导航");
    m.insert("select", "选择");
    m.insert("validate", "确认");
    m.insert("close", "关闭");
    m.insert("new_name", "新名称");
    m.insert("new_value", "新值");
    m.insert("rename", "重命名");
    m.insert("cancel", "取消");
    m.insert("next_result", "下一个");
    m.insert("prev_result", "上一个");
    m.insert("to_variables", "变量");
    m.insert("to_sections", "章节");
    m.insert("edit", "编辑");
    m.insert("add", "添加");
    m.insert("search_label", "搜索");
    m.insert("language", "语言");
    m.insert("select_language", "选择语言");
    m.insert("language_changed", "语言已更改");
    m.insert("help_title", "帮助 - 可用按键");
    m.insert("file_saved", "文件已保存");
    m.insert("file_save_error", "保存文件错误");
    m.insert("saved", "保存");
    m.insert("navigation", "导航");
    m.insert("inline_edition", "内联编辑");
    m.insert("actions", "操作");
    m.insert("commands", "命令");
    m.insert("sections", "章节");
    m.insert("variables", "变量");
    m.insert("editing", "[编辑中]");
    m
}

fn make_pt_pt() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "Pesquisa");
    m.insert("search_active", "Pesquisa");
    m.insert("section_next", "Proxima area");
    m.insert("section_prev", "Area anterior");
    m.insert("move_list", "Mover nas listas");
    m.insert("activate_search", "Ativar pesquisa");
    m.insert("next_prev_result", "Proximo/anterior resultado");
    m.insert("go_result", "Ir para resultado");
    m.insert("quit_search", "Sair da pesquisa");
    m.insert("edit_value", "Editar valor");
    m.insert("rename_key", "Renomear chave");
    m.insert("cancel_edition", "Cancelar edicao");
    m.insert("add_section_var", "Adicionar (seccao/variavel)");
    m.insert("delete", "Eliminar");
    m.insert("comment_uncomment", "Comentar/Descomentar");
    m.insert("quit_no_save", "Sair sem guardar");
    m.insert("save_quit", "Guardar e sair");
    m.insert("help_quit", "Ajuda (q para sair)");
    m.insert("choose", "Escolher");
    m.insert("confirm_delete_section", "CONFIRMAR ELIMINAR SECCAO");
    m.insert("yes", "Sim");
    m.insert("no", "Nao");
    m.insert("navigate", "Navegar");
    m.insert("select", "Selecionar");
    m.insert("validate", "Validar");
    m.insert("close", "Fechar");
    m.insert("new_name", "Novo nome");
    m.insert("new_value", "Novo valor");
    m.insert("rename", "Renomear");
    m.insert("cancel", "Cancelar");
    m.insert("next_result", "Proximo");
    m.insert("prev_result", "Anterior");
    m.insert("to_variables", "Variaveis");
    m.insert("to_sections", "Seccoes");
    m.insert("edit", "Editar");
    m.insert("add", "Adicionar");
    m.insert("search_label", "Pesquisar");
    m.insert("language", "Idioma");
    m.insert("select_language", "Selecionar idioma");
    m.insert("language_changed", "Idioma alterado");
    m.insert("help_title", "Ajuda - Teclas disponiveis");
    m.insert("navigation", "Navegacao");
    m.insert("inline_edition", "Edicao em linha");
    m.insert("actions", "Acoes");
    m.insert("commands", "Comandos");
    m.insert("sections", "SECCOES");
    m.insert("variables", "VARIAVEIS");
    m.insert("editing", "[A editar]");
    m
}

fn make_fi_fi() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "Hae");
    m.insert("search_active", "Hae");
    m.insert("section_next", "Seuraava alue");
    m.insert("section_prev", "Edellinen alue");
    m.insert("move_list", "Siirry listoissa");
    m.insert("activate_search", "Aktivoi haku");
    m.insert("next_prev_result", "Seuraava/edellinen tulos");
    m.insert("go_result", "Siirry tulokseen");
    m.insert("quit_search", "Poistu hausta");
    m.insert("edit_value", "Muokkaa arvoa");
    m.insert("rename_key", "Nimea avain uudelleen");
    m.insert("cancel_edition", "Peruuta muokkaus");
    m.insert("add_section_var", "Lisaa (osio/muuttuja)");
    m.insert("delete", "Poista");
    m.insert("comment_uncomment", "Kommentoi/Kommentoi pois");
    m.insert("quit_no_save", "Poistu tallentamatta");
    m.insert("save_quit", "Tallenna ja poistu");
    m.insert("help_quit", "Ohje (q poistu)");
    m.insert("choose", "Valitse");
    m.insert("confirm_delete_section", "Vahvista osion poisto");
    m.insert("yes", "Kyllä");
    m.insert("no", "Ei");
    m.insert("navigate", "Siirry");
    m.insert("select", "Valitse");
    m.insert("validate", "Vahvista");
    m.insert("close", "Sulje");
    m.insert("new_name", "Uusi nimi");
    m.insert("new_value", "Uusi arvo");
    m.insert("rename", "Nimea uudelleen");
    m.insert("cancel", "Peruuta");
    m.insert("next_result", "Seuraava");
    m.insert("prev_result", "Edellinen");
    m.insert("to_variables", "Muuttujat");
    m.insert("to_sections", "Osio");
    m.insert("edit", "Muokkaa");
    m.insert("add", "Lisaa");
    m.insert("search_label", "Hae");
    m.insert("language", "Kieli");
    m.insert("select_language", "Valitse kieli");
    m.insert("language_changed", "Kieli vaihdettu");
    m.insert("help_title", "Ohje - Kaytettavat nayppaimet");
    m.insert("file_saved", "Tiedosto tallennettu");
    m.insert("file_save_error", "Virhe tallennuksessa");
    m.insert("saved", "Tallenna");
    m.insert("navigation", "Navigointi");
    m.insert("inline_edition", "Rivilinja muokkaus");
    m.insert("actions", "Toiminnot");
    m.insert("commands", "Komennot");
    m.insert("sections", "OSIOT");
    m.insert("variables", "MUUTTUJAT");
    m.insert("editing", "[Muokataan]");
    m
}

fn make_sv_se() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "Sok");
    m.insert("search_active", "Sok");
    m.insert("section_next", "Nasta omrade");
    m.insert("section_prev", "Foregende omrade");
    m.insert("move_list", "Ror dig i listor");
    m.insert("activate_search", "Aktivera sok");
    m.insert("next_prev_result", "Nasta/foregande resultat");
    m.insert("go_result", "Ga till resultat");
    m.insert("quit_search", "Avsluta sok");
    m.insert("edit_value", "Redigera varde");
    m.insert("rename_key", "Dopa om nyckel");
    m.insert("cancel_edition", "Avbryt redigering");
    m.insert("add_section_var", "Lagg till (sektion/variabel)");
    m.insert("delete", "Ta bort");
    m.insert("comment_uncomment", "Kommentera/Avkommentera");
    m.insert("quit_no_save", "Avsluta utan att spara");
    m.insert("save_quit", "Spara och avsluta");
    m.insert("help_quit", "Hjalp (q avsluta)");
    m.insert("choose", "Valj");
    m.insert("confirm_delete_section", "Bekrafta sektionsborttagning");
    m.insert("yes", "Ja");
    m.insert("no", "Nej");
    m.insert("navigate", "Navigera");
    m.insert("select", "Valj");
    m.insert("validate", "Bekrafta");
    m.insert("close", "Stang");
    m.insert("new_name", "Nytt namn");
    m.insert("new_value", "Nytt varde");
    m.insert("rename", "Dopa om");
    m.insert("cancel", "Avbryt");
    m.insert("next_result", "Nasta");
    m.insert("prev_result", "Forega");
    m.insert("to_variables", "Variabler");
    m.insert("to_sections", "Sektioner");
    m.insert("edit", "Redigera");
    m.insert("add", "Lagg till");
    m.insert("search_label", "Sok");
    m.insert("language", "Sprak");
    m.insert("select_language", "Valj sprak");
    m.insert("language_changed", "Sprak andrat");
    m.insert("help_title", "Hjalp - Tillgangliga tangenter");
    m.insert("navigation", "Navigering");
    m.insert("inline_edition", "Redigering");
    m.insert("actions", "Atgarder");
    m.insert("commands", "Kommandon");
    m.insert("sections", "SEKTIONER");
    m.insert("variables", "VARIABLER");
    m.insert("editing", "[Redigerar]");
    m
}

fn make_da_dk() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "Sog");
    m.insert("search_active", "Sog");
    m.insert("section_next", "Naeste omrade");
    m.insert("section_prev", "Forrige omrade");
    m.insert("move_list", "Ryd i lister");
    m.insert("activate_search", "Aktiver sog");
    m.insert("next_prev_result", "Naeste/forrige resultat");
    m.insert("go_result", "Ga til resultat");
    m.insert("quit_search", "Afslut sog");
    m.insert("edit_value", "Rediger vardi");
    m.insert("rename_key", "Omdob nogle");
    m.insert("cancel_edition", "Annuller redigering");
    m.insert("add_section_var", "Tilfoj (sektion/variabel)");
    m.insert("delete", "Slet");
    m.insert("comment_uncomment", "Kommenter/Fjern kommentar");
    m.insert("quit_no_save", "Afslut uden at gemme");
    m.insert("save_quit", "Gem og afslut");
    m.insert("help_quit", "Hjalp (q afslut)");
    m.insert("choose", "Vaeg");
    m.insert("confirm_delete_section", "Bekraeft sektionssletning");
    m.insert("yes", "Ja");
    m.insert("no", "Nej");
    m.insert("navigate", "Naviger");
    m.insert("select", "Vaeg");
    m.insert("validate", "Bekraeft");
    m.insert("close", "Luk");
    m.insert("new_name", "Nyt navn");
    m.insert("new_value", "Ny vardi");
    m.insert("rename", "Omdob");
    m.insert("cancel", "Annuller");
    m.insert("next_result", "Naeste");
    m.insert("prev_result", "Forrige");
    m.insert("to_variables", "Variabler");
    m.insert("to_sections", "Sektioner");
    m.insert("edit", "Rediger");
    m.insert("add", "Tilfoj");
    m.insert("search_label", "Sog");
    m.insert("language", "Sprog");
    m.insert("select_language", "Vaeg sprog");
    m.insert("language_changed", "Sprog aendret");
    m.insert("help_title", "Hjalp - Tilgaengelige taster");
    m.insert("navigation", "Navigation");
    m.insert("inline_edition", "Redigering");
    m.insert("actions", "Handlinger");
    m.insert("commands", "Kommandoer");
    m.insert("sections", "SEKTIONER");
    m.insert("variables", "VARIABLER");
    m.insert("editing", "[Redigerer]");
    m
}

fn make_nb_no() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "Sok");
    m.insert("search_active", "Sok");
    m.insert("section_next", "Neste omrade");
    m.insert("section_prev", "Forrige omrade");
    m.insert("move_list", "Ror i lister");
    m.insert("activate_search", "Aktiver sok");
    m.insert("next_prev_result", "Neste/forrige resultat");
    m.insert("go_result", "Ga til resultat");
    m.insert("quit_search", "Avslutt sok");
    m.insert("edit_value", "Rediger verdi");
    m.insert("rename_key", "Gi nytt navn");
    m.insert("cancel_edition", "Avbryt redigering");
    m.insert("add_section_var", "Legg til (seksjon/variabel)");
    m.insert("delete", "Slett");
    m.insert("comment_uncomment", "Kommenter/Fjern kommentar");
    m.insert("quit_no_save", "Avslutt uten a lagre");
    m.insert("save_quit", "Lagre og avslutt");
    m.insert("help_quit", "Hjalp (q avslutt)");
    m.insert("choose", "Velg");
    m.insert("confirm_delete_section", "Bekreft seksjonssletting");
    m.insert("yes", "Ja");
    m.insert("no", "Nei");
    m.insert("navigate", "Naviger");
    m.insert("select", "Velg");
    m.insert("validate", "Bekreft");
    m.insert("close", "Lukk");
    m.insert("new_name", "Nytt navn");
    m.insert("new_value", "Ny verdi");
    m.insert("rename", "Gi nytt navn");
    m.insert("cancel", "Avbryt");
    m.insert("next_result", "Neste");
    m.insert("prev_result", "Forrige");
    m.insert("to_variables", "Variabler");
    m.insert("to_sections", "Seksjoner");
    m.insert("edit", "Rediger");
    m.insert("add", "Legg til");
    m.insert("search_label", "Sok");
    m.insert("language", "Sprak");
    m.insert("select_language", "Velg sprak");
    m.insert("language_changed", "Sprak endret");
    m.insert("help_title", "Hjalp - Tilgjengelige taster");
    m.insert("navigation", "Navigasjon");
    m.insert("inline_edition", "Redigering");
    m.insert("actions", "Handlinger");
    m.insert("commands", "Kommandoer");
    m.insert("sections", "SEKSJONER");
    m.insert("variables", "VARIABLER");
    m.insert("editing", "[Redigerer]");
    m
}

fn make_th_th() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "ค้นหา");
    m.insert("search_active", "ค้นหา");
    m.insert("section_next", "พื้นที่ถัดไป");
    m.insert("section_prev", "พื้นที่ก่อนหน้า");
    m.insert("move_list", "เลื่อนในรายการ");
    m.insert("activate_search", "เปิดใช้งานการค้นหา");
    m.insert("next_prev_result", "ผลลัพธ์ถัดไป/ก่อนหน้า");
    m.insert("go_result", "ไปยังผลลัพธ์");
    m.insert("quit_search", "ออกจากการค้นหา");
    m.insert("edit_value", "แก้ไขค่า");
    m.insert("rename_key", "เปลี่ยนชื่อคีย์");
    m.insert("cancel_edition", "ยกเลิกการแก้ไข");
    m.insert("add_section_var", "เพิ่ม (ส่วน/ตัวแปร)");
    m.insert("delete", "ลบ");
    m.insert("comment_uncomment", "คอมเมนต์/ยกเลิกคอมเมนต์");
    m.insert("quit_no_save", "ออกโดยไม่บันทึก");
    m.insert("save_quit", "บันทึกและออก");
    m.insert("help_quit", "วิธีใช้ (q ออก)");
    m.insert("choose", "เลือก");
    m.insert("confirm_delete_section", "ยืนยันการลบส่วน");
    m.insert("yes", "ใช่");
    m.insert("no", "ไม่");
    m.insert("navigate", "นำทาง");
    m.insert("select", "เลือก");
    m.insert("validate", "ยืนยัน");
    m.insert("close", "ปิด");
    m.insert("new_name", "ชื่อใหม่");
    m.insert("new_value", "ค่าใหม่");
    m.insert("rename", "เปลี่ยนชื่อ");
    m.insert("cancel", "ยกเลิก");
    m.insert("next_result", "ถัดไป");
    m.insert("prev_result", "ก่อนหน้า");
    m.insert("to_variables", "ตัวแปร");
    m.insert("to_sections", "ส่วน");
    m.insert("edit", "แก้ไข");
    m.insert("add", "เพิ่ม");
    m.insert("search_label", "ค้นหา");
    m.insert("language", "ภาษา");
    m.insert("select_language", "เลือกภาษา");
    m.insert("language_changed", "เปลี่ยนภาษาแล้ว");
    m.insert("help_title", "วิธีใช้ - ปุ่มที่ใช้งานได้");
    m.insert("navigation", "การนำทาง");
    m.insert("inline_edition", "การแก้ไขแบบอินไลน์");
    m.insert("actions", "การดำเนินการ");
    m.insert("commands", "คำสั่ง");
    m.insert("sections", "ส่วน");
    m.insert("variables", "ตัวแปร");
    m.insert("editing", "[กำลังแก้ไข]");
    m
}

fn make_vi_vn() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "Tim kiem");
    m.insert("search_active", "Tim kiem");
    m.insert("section_next", "Khu vuc tiep theo");
    m.insert("section_prev", "Khu vuc truoc do");
    m.insert("move_list", "Di chuyen trong danh sach");
    m.insert("activate_search", "Kich hoat tim kiem");
    m.insert("next_prev_result", "Ket qua tiep theo/truoc do");
    m.insert("go_result", "Di den ket qua");
    m.insert("quit_search", "Thoat tim kiem");
    m.insert("edit_value", "Chinh sua gia tri");
    m.insert("rename_key", "Doi ten khoa");
    m.insert("cancel_edition", "Huy chinh sua");
    m.insert("add_section_var", "Them (phan/mien)");
    m.insert("delete", "Xoa");
    m.insert("comment_uncomment", "Binh luan/Bo binh luan");
    m.insert("quit_no_save", "Thoat khong luu");
    m.insert("save_quit", "Luu va thoat");
    m.insert("help_quit", "Tro giup (q thoat)");
    m.insert("choose", "Chon");
    m.insert("confirm_delete_section", "Xac nhan xoa phan");
    m.insert("yes", "Co");
    m.insert("no", "Khong");
    m.insert("navigate", "Dieu huong");
    m.insert("select", "Chon");
    m.insert("validate", "Xac nhan");
    m.insert("close", "Dong");
    m.insert("new_name", "Ten moi");
    m.insert("new_value", "Gia tri moi");
    m.insert("rename", "Doi ten");
    m.insert("cancel", "Huy");
    m.insert("next_result", "Tiep theo");
    m.insert("prev_result", "Truoc do");
    m.insert("to_variables", "Bien");
    m.insert("to_sections", "Phan");
    m.insert("edit", "Chinh sua");
    m.insert("add", "Them");
    m.insert("search_label", "Tim kiem");
    m.insert("language", "Ngon ngu");
    m.insert("select_language", "Chon ngon ngu");
    m.insert("language_changed", "Da doi ngon ngu");
    m.insert("help_title", "Tro giup - Phim kha dung");
    m.insert("navigation", "Dieu huong");
    m.insert("inline_edition", "Chinh sua noi tuyen");
    m.insert("actions", "Hanh dong");
    m.insert("commands", "Lenh");
    m.insert("sections", "PHAN");
    m.insert("variables", "BIEN");
    m.insert("editing", "[Dang chinh sua]");
    m
}

fn make_km_kh() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "ស្វែងរក");
    m.insert("search_active", "ស្វែងរក");
    m.insert("section_next", "តំបន់បន្ទាប់");
    m.insert("section_prev", "តំបន់មុន");
    m.insert("move_list", "ផ្លាស់ទីក្នុងបញ្ជី");
    m.insert("activate_search", "ធ្វើឱ្យស្វែងរកសកម្ម");
    m.insert("next_prev_result", "លទ្ធផលបន្ទាប់/មុន");
    m.insert("go_result", "ទៅកាន់លទ្ធផល");
    m.insert("quit_search", "ចាកចេញពីការស្វែងរក");
    m.insert("edit_value", "កែសម្រួលតម្លៃ");
    m.insert("rename_key", "ប្តូរឈ្មោះគ្ey");
    m.insert("cancel_edition", "បោះបង់កែសម្រួល");
    m.insert("add_section_var", "បន្ថែម (ផ្នែក/អថេរ)");
    m.insert("delete", "លុប");
    m.insert("comment_uncomment", "វិចារ/លុបវិចារ");
    m.insert("quit_no_save", "ចាកចេញដោយមិនរក្សាទុក");
    m.insert("save_quit", "រក្សាទុកនិងចាកចេញ");
    m.insert("help_quit", "ជំនួយ (q ចាកចេញ)");
    m.insert("choose", "ជ្រើសរើស");
    m.insert("confirm_delete_section", "បញ្ជាក់ការលុបផ្នែក");
    m.insert("yes", "បាទ/ចាស");
    m.insert("no", "ទេ");
    m.insert("navigate", "រុករក");
    m.insert("select", "ជ្រើសរើស");
    m.insert("validate", "បញ្ជាក់");
    m.insert("close", "បិទ");
    m.insert("new_name", "ឈ្មោះថ្មី");
    m.insert("new_value", "តម្លៃថ្មី");
    m.insert("rename", "ប្តូរឈ្មោះ");
    m.insert("cancel", "បោះបង់");
    m.insert("next_result", "បន្ទាប់");
    m.insert("prev_result", "មុន");
    m.insert("to_variables", "អថេរ");
    m.insert("to_sections", "ផ្នែក");
    m.insert("edit", "កែសម្រួល");
    m.insert("add", "បន្ថែម");
    m.insert("search_label", "ស្វែងរក");
    m.insert("language", "ភាសា");
    m.insert("select_language", "ជ្រើសរើសភាសា");
    m.insert("language_changed", "បានផ្លាស់ប្តូរភាសា");
    m.insert("help_title", "ជំនួយ - គ្eyដែលអាចប្រើបាន");
    m.insert("navigation", "ការរុករក");
    m.insert("inline_edition", "កែសម្រួលក្នុងជួរ");
    m.insert("actions", "សកម្មភាព");
    m.insert("commands", "ពាក្យបញ្ជា");
    m.insert("sections", "ផ្នែក");
    m.insert("variables", "អថេរ");
    m.insert("editing", "[កំពុងកែសម្រួល]");
    m
}

fn make_ru_ru() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "ПОИСК");
    m.insert("search_active", "Поиск");
    m.insert("section_next", "Следующая область");
    m.insert("section_prev", "Предыдущая область");
    m.insert("move_list", "Перемещение в списках");
    m.insert("activate_search", "Активировать поиск");
    m.insert("next_prev_result", "Следующий/предыдущий результат");
    m.insert("go_result", "Перейти к результату");
    m.insert("quit_search", "Выйти из поиска");
    m.insert("edit_value", "Редактировать значение");
    m.insert("rename_key", "Переименовать ключ");
    m.insert("cancel_edition", "Отменить редактирование");
    m.insert("add_section_var", "Добавить (секция/переменная)");
    m.insert("delete", "Удалить");
    m.insert("comment_uncomment", "Комментировать/Раскомментировать");
    m.insert("quit_no_save", "Выйти без сохранения");
    m.insert("save_quit", "Сохранить и выйти");
    m.insert("help_quit", "Помощь (q выход)");
    m.insert("choose", "Выбрать");
    m.insert("confirm_delete_section", "ПОДТВЕРДИТЬ УДАЛЕНИЕ СЕКЦИИ");
    m.insert("yes", "Да");
    m.insert("no", "Нет");
    m.insert("navigate", "Навигация");
    m.insert("select", "Выбрать");
    m.insert("validate", "Подтвердить");
    m.insert("close", "Закрыть");
    m.insert("new_name", "Новое имя");
    m.insert("new_value", "Новое значение");
    m.insert("rename", "Переименовать");
    m.insert("cancel", "Отмена");
    m.insert("next_result", "Следующий");
    m.insert("prev_result", "Предыдущий");
    m.insert("to_variables", "Переменные");
    m.insert("to_sections", "Секции");
    m.insert("edit", "Редактировать");
    m.insert("add", "Добавить");
    m.insert("search_label", "Поиск");
    m.insert("language", "Язык");
    m.insert("select_language", "Выбрать язык");
    m.insert("language_changed", "Язык изменен");
    m.insert("help_title", "Помощь - Доступные клавиши");
    m.insert("navigation", "Навигация");
    m.insert("inline_edition", "Строковое редактирование");
    m.insert("actions", "Действия");
    m.insert("commands", "Команды");
    m.insert("sections", "СЕКЦИИ");
    m.insert("variables", "ПЕРЕМЕННЫЕ");
    m.insert("editing", "[Редактирование]");
    m
}

fn make_tr_tr() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "Ara");
    m.insert("search_active", "Ara");
    m.insert("section_next", "Sonraki alan");
    m.insert("section_prev", "Onceki alan");
    m.insert("move_list", "Listelerde gezin");
    m.insert("activate_search", "Aramayi aktiflestir");
    m.insert("next_prev_result", "Sonraki/onceki sonuc");
    m.insert("go_result", "Sonuca git");
    m.insert("quit_search", "Aramadan cik");
    m.insert("edit_value", "Degeri duzenle");
    m.insert("rename_key", "Anahtari yeniden adlandir");
    m.insert("cancel_edition", "Duzenlemeyi iptal et");
    m.insert("add_section_var", "Ekle (bolum/degisken)");
    m.insert("delete", "Sil");
    m.insert("comment_uncomment", "Yorum yap/yorumu kaldir");
    m.insert("quit_no_save", "Kaydetmeden cik");
    m.insert("save_quit", "Kaydet ve cik");
    m.insert("help_quit", "Yardim (q cikis)");
    m.insert("choose", "Sec");
    m.insert("confirm_delete_section", "BOLUM SILMEYI ONAYLA");
    m.insert("yes", "Evet");
    m.insert("no", "Hayir");
    m.insert("navigate", "Gezin");
    m.insert("select", "Sec");
    m.insert("validate", "Onayla");
    m.insert("close", "Kapat");
    m.insert("new_name", "Yeni isim");
    m.insert("new_value", "Yeni deger");
    m.insert("rename", "Yeniden adlandir");
    m.insert("cancel", "Iptal");
    m.insert("next_result", "Sonraki");
    m.insert("prev_result", "Onceki");
    m.insert("to_variables", "Degiskenler");
    m.insert("to_sections", "Bolumler");
    m.insert("edit", "Duzenle");
    m.insert("add", "Ekle");
    m.insert("search_label", "Ara");
    m.insert("language", "Dil");
    m.insert("select_language", "Dil sec");
    m.insert("language_changed", "Dil degistirildi");
    m.insert("help_title", "Yardim - Kullanilabilir tuslar");
    m.insert("navigation", "Gezinme");
    m.insert("inline_edition", "Satir ici duzenleme");
    m.insert("actions", "Eylemler");
    m.insert("commands", "Komutlar");
    m.insert("sections", "BOLUMLER");
    m.insert("variables", "DEGISKENLER");
    m.insert("editing", "[Duzenleniyor]");
    m
}

fn make_am_et() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "ፍለጋ");
    m.insert("search_active", "ፈልግ");
    m.insert("section_next", "ቀጣይ ቦታ");
    m.insert("section_prev", "�.previous ቦታ");
    m.insert("move_list", "በዝርዝሮች ውስጥ ያንቀሳቅሱ");
    m.insert("activate_search", "ፍለጋን አንቃ");
    m.insert("next_prev_result", "ቀጣይ/�.previous ውጤት");
    m.insert("go_result", "ወደ ውጤት ሂድ");
    m.insert("quit_search", "ከፍለጋ ውጣ");
    m.insert("edit_value", "እሴቱን አርም");
    m.insert("rename_key", "ቁልፍ ደግሞ ሰይም");
    m.insert("cancel_edition", "ማርቆስ ማቋረጥ");
    m.insert("add_section_var", "ጨምር (ክፍል/ተለዋዋጭ)");
    m.insert("delete", "ሰርዝ");
    m.insert("comment_uncomment", "አስተያየት/አስተያየት አስወግድ");
    m.insert("quit_no_save", "ያልተቀመጠ ሂድ");
    m.insert("save_quit", "አስቀምጥ እና ውጣ");
    m.insert("help_quit", "እገዛ (q ለመውጣት)");
    m.insert("choose", "ምረጥ");
    m.insert("confirm_delete_section", "የክፍል ሰርዝን አረጋግጥ");
    m.insert("yes", "አዎ");
    m.insert("no", "አይ");
    m.insert("navigate", "በኩል");
    m.insert("select", "ምረጥ");
    m.insert("validate", "አረጋግጥ");
    m.insert("close", "ዝጋ");
    m.insert("new_name", "አዲስ ስም");
    m.insert("new_value", "አዲስ እሴት");
    m.insert("rename", "ደግሞ ሰይም");
    m.insert("cancel", "ማቋረጥ");
    m.insert("next_result", "ቀጣይ");
    m.insert("prev_result", "�.previous");
    m.insert("to_variables", "ተለዋዋጮች");
    m.insert("to_sections", "ክፍሎች");
    m.insert("edit", "አርም");
    m.insert("add", "ጨምር");
    m.insert("search_label", "ፍለጋ");
    m.insert("language", "ቋንቋ");
    m.insert("select_language", "ቋንቋ ምረጥ");
    m.insert("language_changed", "ቋንቋ ተለዋወጠ");
    m.insert("help_title", "እገዛ - የሚገኙ ቁልፎች");
    m.insert("navigation", "አቅጣጫ");
    m.insert("inline_edition", "በረድፍ ማርቆስ");
    m.insert("actions", "እርምጦች");
    m.insert("commands", "ትእዛዞች");
    m.insert("sections", "ክፍሎች");
    m.insert("variables", "ተለዋዋጮች");
    m.insert("editing", "[በማርቆስ ላይ]");
    m
}

fn make_ar_sa() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "بحث");
    m.insert("search_active", "بحث");
    m.insert("section_next", "المنطقة التالية");
    m.insert("section_prev", "المنطقة السابقة");
    m.insert("move_list", "التحرك في القوائم");
    m.insert("activate_search", "تفعيل البحث");
    m.insert("next_prev_result", "النتيجة التالية/السابقة");
    m.insert("go_result", "الذهاب إلى النتيجة");
    m.insert("quit_search", "الخروج من البحث");
    m.insert("edit_value", "تحرير القيمة");
    m.insert("rename_key", "إعادة تسمية المفتاح");
    m.insert("cancel_edition", "إلغاء التحرير");
    m.insert("add_section_var", "إضافة (قسم/متغير)");
    m.insert("delete", "حذف");
    m.insert("comment_uncomment", "تعليق/إلغاء التعليق");
    m.insert("quit_no_save", "الخروج بدون حفظ");
    m.insert("save_quit", "حفظ والخروج");
    m.insert("help_quit", "مساعدة (q للخروج)");
    m.insert("choose", "اختر");
    m.insert("confirm_delete_section", "تأكيد حذف القسم");
    m.insert("yes", "نعم");
    m.insert("no", "لا");
    m.insert("navigate", "تنقل");
    m.insert("select", "اختر");
    m.insert("validate", "تأكيد");
    m.insert("close", "إغلاق");
    m.insert("new_name", "الاسم الجديد");
    m.insert("new_value", "القيمة الجديدة");
    m.insert("rename", "إعادة تسمية");
    m.insert("cancel", "إلغاء");
    m.insert("next_result", "التالي");
    m.insert("prev_result", "السابق");
    m.insert("to_variables", "المتغيرات");
    m.insert("to_sections", "الأقسام");
    m.insert("edit", "تحرير");
    m.insert("add", "إضافة");
    m.insert("search_label", "بحث");
    m.insert("language", "اللغة");
    m.insert("select_language", "اختر اللغة");
    m.insert("language_changed", "تم تغيير اللغة");
    m.insert("help_title", "مساعدة - المفاتيح المتاحة");
    m.insert("navigation", "التنقل");
    m.insert("inline_edition", "التحرير في السطر");
    m.insert("actions", "الإجراءات");
    m.insert("commands", "الأوامر");
    m.insert("sections", "الأقسام");
    m.insert("variables", "المتغيرات");
    m.insert("editing", "[قيد التحرير]");
    m
}

fn make_ko_kr() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "검색");
    m.insert("search_active", "검색");
    m.insert("section_next", "다음 영역");
    m.insert("section_prev", "이전 영역");
    m.insert("move_list", "목록에서 이동");
    m.insert("activate_search", "검색 활성화");
    m.insert("next_prev_result", "다음/이전 결과");
    m.insert("go_result", "결과로 이동");
    m.insert("quit_search", "검색 종료");
    m.insert("edit_value", "값 편집");
    m.insert("rename_key", "키 이름 변경");
    m.insert("cancel_edition", "편집 취소");
    m.insert("add_section_var", "추가 (섹션/변수)");
    m.insert("delete", "삭제");
    m.insert("comment_uncomment", "주석 처리/주석 해제");
    m.insert("quit_no_save", "저장 없이 종료");
    m.insert("save_quit", "저장 후 종료");
    m.insert("help_quit", "도움말 (q 종료)");
    m.insert("choose", "선택");
    m.insert("confirm_delete_section", "섹션 삭제 확인");
    m.insert("yes", "예");
    m.insert("no", "아니오");
    m.insert("navigate", "탐색");
    m.insert("select", "선택");
    m.insert("validate", "확인");
    m.insert("close", "닫기");
    m.insert("new_name", "새 이름");
    m.insert("new_value", "새 값");
    m.insert("rename", "이름 변경");
    m.insert("cancel", "취소");
    m.insert("next_result", "다음");
    m.insert("prev_result", "이전");
    m.insert("to_variables", "변수");
    m.insert("to_sections", "섹션");
    m.insert("edit", "편집");
    m.insert("add", "추가");
    m.insert("search_label", "검색");
    m.insert("language", "언어");
    m.insert("select_language", "언어 선택");
    m.insert("language_changed", "언어가 변경되었습니다");
    m.insert("help_title", "도움말 - 사용 가능한 키");
    m.insert("navigation", "탐색");
    m.insert("inline_edition", "인라인 편집");
    m.insert("actions", "작업");
    m.insert("commands", "명령");
    m.insert("sections", "섹션");
    m.insert("variables", "변수");
    m.insert("editing", "[편집 중]");
    m
}

fn make_id_id() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "Cari");
    m.insert("search_active", "Cari");
    m.insert("section_next", "Area berikutnya");
    m.insert("section_prev", "Area sebelumnya");
    m.insert("move_list", "Bergerak dalam daftar");
    m.insert("activate_search", "Aktifkan pencarian");
    m.insert("next_prev_result", "Hasil berikutnya/sebelumnya");
    m.insert("go_result", "Pergi ke hasil");
    m.insert("quit_search", "Keluar dari pencarian");
    m.insert("edit_value", "Edit nilai");
    m.insert("rename_key", "Ubah nama kunci");
    m.insert("cancel_edition", "Batalkan penyuntingan");
    m.insert("add_section_var", "Tambah (bagian/variabel)");
    m.insert("delete", "Hapus");
    m.insert("comment_uncomment", "Komentari/Batalkan komentar");
    m.insert("quit_no_save", "Keluar tanpa menyimpan");
    m.insert("save_quit", "Simpan dan keluar");
    m.insert("help_quit", "Bantuan (q keluar)");
    m.insert("choose", "Pilih");
    m.insert("confirm_delete_section", "KONFIRMASI HAPUS BAGIAN");
    m.insert("yes", "Ya");
    m.insert("no", "Tidak");
    m.insert("navigate", "Navigasi");
    m.insert("select", "Pilih");
    m.insert("validate", "Konfirmasi");
    m.insert("close", "Tutup");
    m.insert("new_name", "Nama baru");
    m.insert("new_value", "Nilai baru");
    m.insert("rename", "Ubah nama");
    m.insert("cancel", "Batal");
    m.insert("next_result", "Berikutnya");
    m.insert("prev_result", "Sebelumnya");
    m.insert("to_variables", "Variabel");
    m.insert("to_sections", "Bagian");
    m.insert("edit", "Edit");
    m.insert("add", "Tambah");
    m.insert("search_label", "Cari");
    m.insert("language", "Bahasa");
    m.insert("select_language", "Pilih bahasa");
    m.insert("language_changed", "Bahasa diubah");
    m.insert("help_title", "Bantuan - Tombol yang tersedia");
    m.insert("navigation", "Navigasi");
    m.insert("inline_edition", "Penyuntingan sebaris");
    m.insert("actions", "Tindakan");
    m.insert("commands", "Perintah");
    m.insert("sections", "BAGIAN");
    m.insert("variables", "VARIABEL");
    m.insert("editing", "[Menyunting]");
    m
}

fn make_el_gr() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "ΑΝΑΖΗΤΗΣΗ");
    m.insert("search_active", "Αναζήτηση");
    m.insert("section_next", "Επόμενη περιοχή");
    m.insert("section_prev", "Προηγούμενη περιοχή");
    m.insert("move_list", "Μετακίνηση στις λίστες");
    m.insert("activate_search", "Ενεργοποίηση αναζήτησης");
    m.insert("next_prev_result", "Επόμενο/προηγούμενο αποτέλεσμα");
    m.insert("go_result", "Μετάβαση στο αποτέλεσμα");
    m.insert("quit_search", "Έξοδος από αναζήτηση");
    m.insert("edit_value", "Επεξεργασία τιμής");
    m.insert("rename_key", "Μετονομασία κλειδιού");
    m.insert("cancel_edition", "Ακύρωση επεξεργασίας");
    m.insert("add_section_var", "Προσθήκη (ενότητα/μεταβλητή)");
    m.insert("delete", "Διαγραφή");
    m.insert("comment_uncomment", "Σχολιασμός/Κατάργηση σχολιασμού");
    m.insert("quit_no_save", "Έξοδος χωρίς αποθήκευση");
    m.insert("save_quit", "Αποθήκευση και έξοδος");
    m.insert("help_quit", "Βοήθεια (q έξοδος)");
    m.insert("choose", "Επιλογή");
    m.insert("confirm_delete_section", "ΕΠΙΒΕΒΑΙΩΣΗ ΔΙΑΓΡΑΦΗΣ ΕΝΟΤΗΤΑΣ");
    m.insert("yes", "Ναι");
    m.insert("no", "Όχι");
    m.insert("navigate", "Πλοήγηση");
    m.insert("select", "Επιλογή");
    m.insert("validate", "Επιβεβαίωση");
    m.insert("close", "Κλείσιμο");
    m.insert("new_name", "Νέο όνομα");
    m.insert("new_value", "Νέα τιμή");
    m.insert("rename", "Μετονομασία");
    m.insert("cancel", "Ακύρωση");
    m.insert("next_result", "Επόμενο");
    m.insert("prev_result", "Προηγούμενο");
    m.insert("to_variables", "Μεταβλητές");
    m.insert("to_sections", "Ενότητες");
    m.insert("edit", "Επεξεργασία");
    m.insert("add", "Προσθήκη");
    m.insert("search_label", "Αναζήτηση");
    m.insert("language", "Γλώσσα");
    m.insert("select_language", "Επιλογή γλώσσας");
    m.insert("language_changed", "Η γλώσσα άλλαξε");
    m.insert("help_title", "Βοήθεια - Διαθέσιμα πλήκτρα");
    m.insert("navigation", "Πλοήγηση");
    m.insert("inline_edition", "Επεξεργασία εν σειρά");
    m.insert("actions", "Ενέργειες");
    m.insert("commands", "Εντολές");
    m.insert("sections", "ΕΝΟΤΗΤΕΣ");
    m.insert("variables", "ΜΕΤΑΒΛΗΤΕΣ");
    m.insert("editing", "[Επεξεργασία]");
    m
}

fn make_ro_ro() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "Cauta");
    m.insert("search_active", "Cauta");
    m.insert("section_next", "Zona urmatoare");
    m.insert("section_prev", "Zona anterioara");
    m.insert("move_list", "Mutare in liste");
    m.insert("activate_search", "Activeaza cautarea");
    m.insert("next_prev_result", "Rezultatul urmator/anterior");
    m.insert("go_result", "Mergi la rezultat");
    m.insert("quit_search", "Iesire din cautare");
    m.insert("edit_value", "Editeaza valoarea");
    m.insert("rename_key", "Redenumeste cheia");
    m.insert("cancel_edition", "Anuleaza editarea");
    m.insert("add_section_var", "Adauga (sectiune/variabila)");
    m.insert("delete", "Sterge");
    m.insert("comment_uncomment", "Comenteaza/Decomenteaza");
    m.insert("quit_no_save", "Iesire fara salvare");
    m.insert("save_quit", "Salveaza si iese");
    m.insert("help_quit", "Ajutor (q pentru iesire)");
    m.insert("choose", "Alege");
    m.insert("confirm_delete_section", "CONFIRMA STERGEREA SECTIUNII");
    m.insert("yes", "Da");
    m.insert("no", "Nu");
    m.insert("navigate", "Navigare");
    m.insert("select", "Selecteaza");
    m.insert("validate", "Confirma");
    m.insert("close", "Inchide");
    m.insert("new_name", "Nume nou");
    m.insert("new_value", "Valoare noua");
    m.insert("rename", "Redenumeste");
    m.insert("cancel", "Anuleaza");
    m.insert("next_result", "Urmatorul");
    m.insert("prev_result", "Anteriorul");
    m.insert("to_variables", "Variabile");
    m.insert("to_sections", "Sectiuni");
    m.insert("edit", "Editeaza");
    m.insert("add", "Adauga");
    m.insert("search_label", "Cauta");
    m.insert("language", "Limba");
    m.insert("select_language", "Selecteaza limba");
    m.insert("language_changed", "Limba schimbata");
    m.insert("help_title", "Ajutor - Taste disponibile");
    m.insert("navigation", "Navigare");
    m.insert("inline_edition", "Editare pe rand");
    m.insert("actions", "Actiuni");
    m.insert("commands", "Comenzi");
    m.insert("sections", "SECTIUNI");
    m.insert("variables", "VARIABILE");
    m.insert("editing", "[Se editeaza]");
    m
}

fn make_et_ee() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "Otsi");
    m.insert("search_active", "Otsi");
    m.insert("section_next", "Jargmine ala");
    m.insert("section_prev", "Eelmine ala");
    m.insert("move_list", "Liigu nimekirjades");
    m.insert("activate_search", "Aktiveeri otsing");
    m.insert("next_prev_result", "Jargmine/eelmine tulemus");
    m.insert("go_result", "Mine tulemuse juurde");
    m.insert("quit_search", "Välju otsingust");
    m.insert("edit_value", "Muuda väärtust");
    m.insert("rename_key", "Nimeta võti ümber");
    m.insert("cancel_edition", "Tühista muutmine");
    m.insert("add_section_var", "Lisa (osa/muutuja)");
    m.insert("delete", "Kustuta");
    m.insert("comment_uncomment", "Kommenteeri/Eemalda kommentaar");
    m.insert("quit_no_save", "Välju ilma salvestamata");
    m.insert("save_quit", "Salvesta ja välju");
    m.insert("help_quit", "Abi (q välju)");
    m.insert("choose", "Vali");
    m.insert("confirm_delete_section", "KINNITA OSA KUSTUTAMINE");
    m.insert("yes", "Jah");
    m.insert("no", "Ei");
    m.insert("navigate", "Navigeeri");
    m.insert("select", "Vali");
    m.insert("validate", "Kinnita");
    m.insert("close", "Sulge");
    m.insert("new_name", "Uus nimi");
    m.insert("new_value", "Uus väärtus");
    m.insert("rename", "Nimeta ümber");
    m.insert("cancel", "Tühista");
    m.insert("next_result", "Järgmine");
    m.insert("prev_result", "Eelmine");
    m.insert("to_variables", "Muutujad");
    m.insert("to_sections", "Osad");
    m.insert("edit", "Muuda");
    m.insert("add", "Lisa");
    m.insert("search_label", "Otsi");
    m.insert("language", "Keel");
    m.insert("select_language", "Vali keel");
    m.insert("language_changed", "Keel muutus");
    m.insert("help_title", "Abi - Saadaolevad klahvid");
    m.insert("navigation", "Navigatsioon");
    m.insert("inline_edition", "Reasisene muutmine");
    m.insert("actions", "Toimingud");
    m.insert("commands", "Käsud");
    m.insert("sections", "OSAD");
    m.insert("variables", "MUUTUJAD");
    m.insert("editing", "[Muutmisel]");
    m
}

fn make_lv_lv() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "Meklet");
    m.insert("search_active", "Meklet");
    m.insert("section_next", "Nakamā joma");
    m.insert("section_prev", "Iepriekšeja joma");
    m.insert("move_list", "Pārvietoties sarakstos");
    m.insert("activate_search", "Aktivēt meklēšanu");
    m.insert("next_prev_result", "Nakamais/iepriekšejs rezultāts");
    m.insert("go_result", "Doties uz rezultātu");
    m.insert("quit_search", "Iziet no meklēšanas");
    m.insert("edit_value", "Rediģēt vērtību");
    m.insert("rename_key", "Pārdēvēt atslēgu");
    m.insert("cancel_edition", "Atcelt rediģēšanu");
    m.insert("add_section_var", "Pievienot (sadaļu/mainīgo)");
    m.insert("delete", "Dzēst");
    m.insert("comment_uncomment", "Komentēt/Noņemt komentāru");
    m.insert("quit_no_save", "Iziet nesaglabājot");
    m.insert("save_quit", "Saglabāt un iziet");
    m.insert("help_quit", "Palīdzība (q iziet)");
    m.insert("choose", "Izvēlēties");
    m.insert("confirm_delete_section", "APSTIPRINĀT SADAĻAS DZĒŠANU");
    m.insert("yes", "Jā");
    m.insert("no", "Nē");
    m.insert("navigate", "Navigēt");
    m.insert("select", "Izvēlēties");
    m.insert("validate", "Apstiprināt");
    m.insert("close", "Aizvērt");
    m.insert("new_name", "Jauns nosaukums");
    m.insert("new_value", "Jauna vērtība");
    m.insert("rename", "Pārdēvēt");
    m.insert("cancel", "Atcelt");
    m.insert("next_result", "Nākamais");
    m.insert("prev_result", "Iepriekšējais");
    m.insert("to_variables", "Mainīgie");
    m.insert("to_sections", "Sadaļas");
    m.insert("edit", "Rediģēt");
    m.insert("add", "Pievienot");
    m.insert("search_label", "Meklēt");
    m.insert("language", "Valoda");
    m.insert("select_language", "Izvēlēties valodu");
    m.insert("language_changed", "Valoda mainīta");
    m.insert("help_title", "Palīdzība - Pieejamās taustiņi");
    m.insert("navigation", "Navigācija");
    m.insert("inline_edition", "Rindas rediģēšana");
    m.insert("actions", "Darbības");
    m.insert("commands", "Komandas");
    m.insert("sections", "SADAĻAS");
    m.insert("variables", "MAINĪGIE");
    m.insert("editing", "[Rediģē]");
    m
}

fn make_lt_lt() -> TranslationMap {
    let mut m = HashMap::new();
    m.insert("app_title", "ini-editor");
    m.insert("search", "Ieskoti");
    m.insert("search_active", "Ieskoti");
    m.insert("section_next", "Kitas sritis");
    m.insert("section_prev", "Ankstesne sritis");
    m.insert("move_list", "Judeti sarasuose");
    m.insert("activate_search", "Aktyvinti ieskojima");
    m.insert("next_prev_result", "Kitas/ankstesnis rezultatas");
    m.insert("go_result", "Eiti i rezultata");
    m.insert("quit_search", "Išeiti iš ieskojimo");
    m.insert("edit_value", "Redaguoti reikšme");
    m.insert("rename_key", "Pervadinti rakta");
    m.insert("cancel_edition", "Atšaukti redagavima");
    m.insert("add_section_var", "Prideti (skilti/kintamaji)");
    m.insert("delete", "Ištrinti");
    m.insert("comment_uncomment", "Kurti komentarą/Šalinti komentarą");
    m.insert("quit_no_save", "Išeiti neišsaugojus");
    m.insert("save_quit", "Išsaugoti ir išeiti");
    m.insert("help_quit", "Pagalba (q išeiti)");
    m.insert("choose", "Pasirinkti");
    m.insert("confirm_delete_section", "PATVIRTINTI SKILTIES IŠTRYNIMĄ");
    m.insert("yes", "Taip");
    m.insert("no", "Ne");
    m.insert("navigate", "Naršyti");
    m.insert("select", "Pasirinkti");
    m.insert("validate", "Patvirtinti");
    m.insert("close", "Uždaryti");
    m.insert("new_name", "Naujas vardas");
    m.insert("new_value", "Nauja reikšme");
    m.insert("rename", "Pervadinti");
    m.insert("cancel", "Atšaukti");
    m.insert("next_result", "Kitas");
    m.insert("prev_result", "Ankstesnis");
    m.insert("to_variables", "Kintamieji");
    m.insert("to_sections", "Skiltys");
    m.insert("edit", "Redaguoti");
    m.insert("add", "Prideti");
    m.insert("search_label", "Ieskoti");
    m.insert("language", "Kalba");
    m.insert("select_language", "Pasirinkti kalba");
    m.insert("language_changed", "Kalba pakeista");
    m.insert("help_title", "Pagalba - Galimi klavisai");
    m.insert("navigation", "Navigacija");
    m.insert("inline_edition", "Eilutės redagavimas");
    m.insert("actions", "Veiksmai");
    m.insert("commands", "Komandos");
    m.insert("sections", "SKILTYS");
    m.insert("variables", "KINTAMIEJI");
    m.insert("editing", "[Redaguojama]");
    m
}

static TRANSLATIONS: Lazy<HashMap<&'static str, TranslationMap>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("fr_FR", make_fr_fr());
    m.insert("en_US", make_en_us());
    m.insert("es_ES", make_es_es());
    m.insert("de_DE", make_de_de());
    m.insert("it_IT", make_it_it());
    m.insert("pt_BR", make_pt_br());
    m.insert("pt_PT", make_pt_pt());
    m.insert("fi_FI", make_fi_fi());
    m.insert("sv_SE", make_sv_se());
    m.insert("da_DK", make_da_dk());
    m.insert("nb_NO", make_nb_no());
    m.insert("th_TH", make_th_th());
    m.insert("vi_VN", make_vi_vn());
    m.insert("km_KH", make_km_kh());
    m.insert("ja_JP", make_ja_jp());
    m.insert("zh_CN", make_zh_cn());
    m.insert("ru_RU", make_ru_ru());
    m.insert("tr_TR", make_tr_tr());
    m.insert("am_ET", make_am_et());
    m.insert("ar_SA", make_ar_sa());
    m.insert("ko_KR", make_ko_kr());
    m.insert("id_ID", make_id_id());
    m.insert("el_GR", make_el_gr());
    m.insert("ro_RO", make_ro_ro());
    m.insert("et_EE", make_et_ee());
    m.insert("lv_LV", make_lv_lv());
    m.insert("lt_LT", make_lt_lt());
    m
});

pub fn get_translations(lang: &str) -> &'static TranslationMap {
    TRANSLATIONS
        .get(lang)
        .unwrap_or_else(|| TRANSLATIONS.get("fr_FR").unwrap())
}

pub fn t(key: &str) -> String {
    let lang = CURRENT_LANGUAGE.read().unwrap();
    let translations = get_translations(&lang);
    translations.get(key).unwrap_or(&key).to_string()
}

#[allow(dead_code)]
pub fn tl(key: &str) -> String {
    t(key)
}

pub fn set_language(lang: &str) {
    let mut current = CURRENT_LANGUAGE.write().unwrap();
    *current = lang.to_string();
}

pub fn get_current_language() -> String {
    CURRENT_LANGUAGE.read().unwrap().clone()
}

pub fn is_valid_language(lang: &str) -> bool {
    AVAILABLE_LANGUAGES.iter().any(|l| l.code == lang)
}
