use serde::Deserialize;
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::process::Command;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Request {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

fn respond(id: &Value, result: Value) {
    let response = json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result
    });
    let out = io::stdout();
    let mut handle = out.lock();
    serde_json::to_writer(&mut handle, &response).unwrap();
    handle.write_all(b"\n").unwrap();
    handle.flush().unwrap();
}

fn handle_tool_call(params: Value, require_confirmation: bool) -> Value {
    let name = params["name"].as_str().unwrap_or("");
    let args = params["arguments"].as_object();

    let json_args_str = match args {
        Some(a) => serde_json::to_string_pretty(a).unwrap_or_default(),
        None => "Aucun argument".to_string(),
    };

    if require_confirmation {
        let description = format!(
            "L'IA souhaite exécuter l'outil : '{}'\n\nArguments :\n{}",
            name, json_args_str
        );

        let confirmed = rfd::MessageDialog::new()
            .set_level(rfd::MessageLevel::Warning)
            .set_title("Exécution d'une commande système")
            .set_description(&description)
            .set_buttons(rfd::MessageButtons::YesNo)
            .show();

        if confirmed != rfd::MessageDialogResult::Yes {
            return json!({
                "isError": true,
                "content": [{ "type": "text", "text": "Action annulée par l'utilisateur." }]
            });
        }
    }

    match name {
        "git_status" => {
            let custom_path = args.and_then(|a| a.get("path")).and_then(|p| p.as_str())
                               .unwrap_or("C:\\Users\\ferta\\Documents\\GitHub\\MCP-server");
            
            let mut cmd = Command::new("git");
            cmd.arg("status").current_dir(custom_path);
            
            let output = cmd.output().unwrap();
            let mut result_str = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr_str = String::from_utf8_lossy(&output.stderr).to_string();
            let info = format!("Dossier courant d'exécution: {}", custom_path);
            
            if !stderr_str.is_empty() {
                result_str = format!("{}\n{}\nErreur Git:\n{}", info, result_str, stderr_str);
            } else {
                result_str = format!("{}\n----\n{}", info, result_str);
            }
            if result_str.trim().is_empty() {
                result_str = "(Sortie vide)".to_string();
            }
            json!({ "content": [{ "type": "text", "text": result_str }] })
        }
        "update_file" => {
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
        "read_file" => {
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
        "replace_text_in_file" => {
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
        "run_command" => {
            if let Some(a) = args {
                let cmd_str = a.get("command").and_then(|c| c.as_str()).unwrap_or("");
                let cwd = a.get("cwd").and_then(|c| c.as_str()).unwrap_or("C:\\Users\\ferta\\Documents\\GitHub\\MCP-server");

                let mut cmd = Command::new("powershell");
                cmd.arg("-Command").arg(cmd_str).current_dir(cwd);

                match cmd.output() {
                    Ok(output) => {
                        let mut result_str = String::from_utf8_lossy(&output.stdout).to_string();
                        let stderr_str = String::from_utf8_lossy(&output.stderr).to_string();

                        let info = format!("Dossier: {}\nCommande: {}", cwd, cmd_str);

                        if !stderr_str.is_empty() {
                            result_str = format!("{}\n{}\nErreur/Warn:\n{}", info, result_str, stderr_str);
                        } else {
                            result_str = format!("{}\n----\n{}", info, result_str);
                        }
                        if result_str.trim().is_empty() {
                            result_str = "(Sortie vide)".to_string();
                        }
                        json!({ "content": [{ "type": "text", "text": result_str }] })
                    },
                    Err(e) => {
                        json!({ "isError": true, "content": [{ "type": "text", "text": format!("Erreur d'exécution: {}", e) }] })
                    }
                }
            } else {
                json!({ "isError": true, "content": [{ "type": "text", "text": "Arguments manquants" }] })
            }
        }
        _ => json!({ "isError": true, "content": [{ "type": "text", "text": "Outil inconnu" }] })
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let require_confirmation = !args.contains(&"--auto-approve".to_string());

    let stdin = io::stdin();
    
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        
        if line.trim().is_empty() { continue; }

        if let Ok(req) = serde_json::from_str::<Request>(&line) {
            match req.method.as_str() {
                "initialize" => {
                    if let Some(id) = req.id {
                        respond(&id, json!({
                            "protocolVersion": "2024-11-05",
                            "serverInfo": { "name": "local-rust-mcp", "version": "0.1.0" },
                            "capabilities": {
                                "tools": {},
                                "resources": {}
                            }
                        }));
                    }
                }
                "notifications/initialized" => {}
                "tools/list" => {
                    if let Some(id) = req.id {
                        respond(&id, json!({
                            "tools": [
                                {
                                    "name": "git_status",
                                    "description": "Lance 'git status' pour voir les modifications.",
                                    "inputSchema": { "type": "object", "properties": {} }
                                },
                                {
                                    "name": "update_file",
                                    "description": "Écrit ou remplace ENTIÈREMENT le contenu d'un fichier existant.",
                                    "inputSchema": {
                                        "type": "object",
                                        "required": ["path", "content"],
                                        "properties": {
                                            "path": { "type": "string", "description": "Chemin du fichier" },
                                            "content": { "type": "string", "description": "Nouveau contenu complet du fichier" }
                                        }
                                    }
                                },
                                {
                                    "name": "read_file",
                                    "description": "Lit le contenu d'un fichier et le renvoie.",
                                    "inputSchema": {
                                        "type": "object",
                                        "required": ["path"],
                                        "properties": {
                                            "path": { "type": "string", "description": "Chemin absolu ou relatif du fichier à lire" }
                                        }
                                    }
                                },
                                {
                                    "name": "replace_text_in_file",
                                    "description": "Cherche un bloc de texte précis dans un fichier et le remplace par un nouveau. Très utile pour modifier ou supprimer une portion de code (si new_text est vide) sans réécrire tout le fichier.",
                                    "inputSchema": {
                                        "type": "object",
                                        "required": ["path", "old_text", "new_text"],
                                        "properties": {
                                            "path": { "type": "string", "description": "Chemin du fichier" },
                                            "old_text": { "type": "string", "description": "Le texte exact à remplacer" },
                                            "new_text": { "type": "string", "description": "Le nouveau texte. Peut être vide '' pour une suppression." }
                                        }
                                    }
                                },
                                {
                                    "name": "run_command",
                                    "description": "Exécute une commande PowerShell dans un terminal (ex: ls, dir). Attention: la commande 'cd' ne sauvegarde pas son état pour l'appel suivant (utilisez l'argument 'cwd' pour définir le dossier).",
                                    "inputSchema": {
                                        "type": "object",
                                        "required": ["command"],
                                        "properties": {
                                            "command": { "type": "string", "description": "La commande à exécuter" },
                                            "cwd": { "type": "string", "description": "Le dossier cible depuis lequel lancer la commande" }
                                        }
                                    }
                                }
                            ]
                        }));
                    }
                }
                "tools/call" => {
                    if let (Some(id), Some(params)) = (req.id, req.params) {
                        let result = handle_tool_call(params, require_confirmation);
                        respond(&id, result);
                    }
                }
                "resources/list" => {}
                "resources/read" => {}
                _ => {}
            }
        }
    }
    Ok(())
}