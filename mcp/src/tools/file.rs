use serde_json::{json, Value};

pub fn handle_read_file(args: Option<&serde_json::Map<String, Value>>) -> Value {
    if let Some(a) = args {
        let path = a.get("path").and_then(|p| p.as_str()).unwrap_or("");
        match std::fs::read_to_string(path) {
            Ok(content) => json!({ "content": [{ "type": "text", "text": content }] }),
            Err(e) => json!({ "isError": true, "content": [{ "type": "text", "text": format!("Erreur de lecture du fichier {} : {}", path, e) }] })
        }
    } else {
        json!({ "isError": true, "content": [{ "type": "text", "text": "Arguments manquants" }] })
    }
}

pub fn handle_write_file(args: Option<&serde_json::Map<String, Value>>) -> Value {
    if let Some(a) = args {
        let path = a.get("path").and_then(|p| p.as_str()).unwrap_or("");
        let content = a.get("content").and_then(|c| c.as_str()).unwrap_or("");
        if let Err(e) = std::fs::write(path, content) {
             json!({ "isError": true, "content": [{ "type": "text", "text": format!("Erreur d'écriture : {}", e) }] })
        } else {
             json!({ "content": [{ "type": "text", "text": format!("Fichier {} modifié en entier.", path) }] })
        }
    } else {
        json!({ "isError": true, "content": [{ "type": "text", "text": "Arguments manquants" }] })
    }
}

pub fn handle_replace_text_in_file(args: Option<&serde_json::Map<String, Value>>) -> Value {
    if let Some(a) = args {
        let path = a.get("path").and_then(|p| p.as_str()).unwrap_or("");
        let old_text = a.get("old_text").and_then(|t| t.as_str()).unwrap_or("");
        let new_text = a.get("new_text").and_then(|t| t.as_str()).unwrap_or("");

        match std::fs::read_to_string(path) {
            Ok(content) => {
                if content.contains(old_text) {
                    let new_content = content.replace(old_text, new_text);
                    if let Err(e) = std::fs::write(path, new_content) {
                        json!({ "isError": true, "content": [{ "type": "text", "text": format!("Erreur d'écriture : {}", e) }] })
                    } else {
                        json!({ "content": [{ "type": "text", "text": format!("Texte remplacé avec succès dans le fichier {}.", path) }] })
                    }
                } else {
                    json!({ "isError": true, "content": [{ "type": "text", "text": "Le texte spécifié (old_text) n'a pas été trouvé dans le fichier, impossible de le remplacer." }] })
                }
            },
            Err(e) => json!({ "isError": true, "content": [{ "type": "text", "text": format!("Erreur de lecture du fichier {} : {}", path, e) }] })
        }
    } else {
        json!({ "isError": true, "content": [{ "type": "text", "text": "Arguments manquants" }] })
    }
}

pub fn handle_rename_file(args: Option<&serde_json::Map<String, Value>>) -> Value {
    if let Some(a) = args {
        let old_path = a.get("old_path").and_then(|p| p.as_str()).unwrap_or("");
        let new_path = a.get("new_path").and_then(|p| p.as_str()).unwrap_or("");

        if old_path.is_empty() || new_path.is_empty() {
             json!({ "isError": true, "content": [{ "type": "text", "text": "Les arguments 'old_path' et 'new_path' sont obligatoires." }] })
        } else if let Err(e) = std::fs::rename(old_path, new_path) {
             json!({ "isError": true, "content": [{ "type": "text", "text": format!("Erreur lors du renommage : {}", e) }] })
        } else {
             json!({ "content": [{ "type": "text", "text": format!("Fichier '{}' renommé en '{}' avec succès.", old_path, new_path) }] })
        }
    } else {
        json!({ "isError": true, "content": [{ "type": "text", "text": "Arguments manquants" }] })
    }
}