use serde::Deserialize;
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

mod tools;

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
        "git_add" => tools::git::handle_git_add(args),
        "git_commit" => tools::git::handle_git_commit(args),
        "git_status" => tools::git::handle_git_status(args),
        "update_file" => tools::file::handle_update_file(args),
        "read_file" => tools::file::handle_read_file(args),
        "replace_text_in_file" => tools::file::handle_replace_text_in_file(args),
        "run_command" => tools::cmd::handle_run_command(args),
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
                                    "name": "git_add",
                                    "description": "Ajoute des fichiers à l'index Git (git add).",
                                    "inputSchema": {
                                        "type": "object",
                                        "required": ["files"],
                                        "properties": {
                                            "path": { "type": "string", "description": "Chemin du dépôt (optionnel)" },
                                            "files": { "type": "string", "description": "Fichiers à ajouter (ex: '.', 'src/main.rs')" }
                                        }
                                    }
                                },
                                {
                                    "name": "git_commit",
                                    "description": "Crée un commit avec les modifications indexées (git commit -m).",
                                    "inputSchema": {
                                        "type": "object",
                                        "required": ["message"],
                                        "properties": {
                                            "path": { "type": "string", "description": "Chemin du dépôt (optionnel)" },
                                            "message": { "type": "string", "description": "Message du commit" }
                                        }
                                    }
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