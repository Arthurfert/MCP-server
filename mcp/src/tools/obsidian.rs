use serde_json::{json, Value};
use std::process::Command;

fn run_obsidian_cmd(args: Vec<String>) -> Value {
    let mut cmd = Command::new("obsidian");
    for arg in args.iter() {
        cmd.arg(arg);
    }

    match cmd.output() {
        Ok(output) => {
            let stdout_str = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr_str = String::from_utf8_lossy(&output.stderr).to_string();

            if !output.status.success() {
                return json!({ "isError": true, "content": [{ "type": "text", "text": format!("Erreur ({}): {}", output.status, stderr_str) }] });
            }

            let mut result_str = stdout_str;
            if !stderr_str.is_empty() {
                result_str.push_str("\n\nStderr:\n");
                result_str.push_str(&stderr_str);
            }
            if result_str.trim().is_empty() {
                result_str = "Commande exécutée avec succès.".to_string();
            }

            json!({ "content": [{ "type": "text", "text": result_str }] })
        },
        Err(e) => {
            json!({ "isError": true, "content": [{ "type": "text", "text": format!("Erreur d'exécution de 'obsidian': {}", e) }] })
        }
    }
}

pub fn handle_obsidian_create(args: Option<&serde_json::Map<String, Value>>) -> Value {
    if let Some(a) = args {
        let mut cmd_args = vec!["create".to_string()];
        
        if let Some(name) = a.get("name").and_then(|v| v.as_str()) {
            cmd_args.push(format!("name={}", name));
        } else if let Some(path) = a.get("path").and_then(|v| v.as_str()) {
            cmd_args.push(format!("path={}", path));
        } else {
            return json!({ "isError": true, "content": [{ "type": "text", "text": "Le paramètre 'name' ou 'path' est obligatoire." }] });
        }

        if let Some(content) = a.get("content").and_then(|v| v.as_str()) {
            // Replace newlines with \n since the CLI documentation says: "Use \n for newline, \t for tab in content values"
            let escaped_content = content.replace("\n", "\\n").replace("\t", "\\t");
            cmd_args.push(format!("content={}", escaped_content));
        }
        
        if let Some(vault) = a.get("vault").and_then(|v| v.as_str()) {
            cmd_args.push(format!("vault={}", vault));
        }
        
        if let Some(overwrite) = a.get("overwrite").and_then(|v| v.as_bool()) {
            if overwrite {
                cmd_args.push("overwrite".to_string());
            }
        }
        
        run_obsidian_cmd(cmd_args)
    } else {
        json!({ "isError": true, "content": [{ "type": "text", "text": "Arguments requis manquants" }] })
    }
}

pub fn handle_obsidian_append(args: Option<&serde_json::Map<String, Value>>) -> Value {
    if let Some(a) = args {
        let mut cmd_args = vec!["append".to_string()];
        
        if let Some(file) = a.get("file").and_then(|v| v.as_str()) {
            cmd_args.push(format!("file={}", file));
        } else if let Some(path) = a.get("path").and_then(|v| v.as_str()) {
            cmd_args.push(format!("path={}", path));
        } else {
            return json!({ "isError": true, "content": [{ "type": "text", "text": "Le paramètre 'file' ou 'path' est obligatoire." }] });
        }

        if let Some(content) = a.get("content").and_then(|v| v.as_str()) {
            let escaped_content = content.replace("\n", "\\n").replace("\t", "\\t");
            cmd_args.push(format!("content={}", escaped_content));
        } else {
            return json!({ "isError": true, "content": [{ "type": "text", "text": "Le paramètre 'content' est obligatoire." }] });
        }
        
        if let Some(vault) = a.get("vault").and_then(|v| v.as_str()) {
            cmd_args.push(format!("vault={}", vault));
        }
        
        run_obsidian_cmd(cmd_args)
    } else {
        json!({ "isError": true, "content": [{ "type": "text", "text": "Arguments requis manquants" }] })
    }
}

pub fn handle_obsidian_read(args: Option<&serde_json::Map<String, Value>>) -> Value {
    if let Some(a) = args {
        let mut cmd_args = vec!["read".to_string()];
        
        if let Some(file) = a.get("file").and_then(|v| v.as_str()) {
            cmd_args.push(format!("file={}", file));
        } else if let Some(path) = a.get("path").and_then(|v| v.as_str()) {
            cmd_args.push(format!("path={}", path));
        } else {
            return json!({ "isError": true, "content": [{ "type": "text", "text": "Le paramètre 'file' ou 'path' est obligatoire." }] });
        }
        
        if let Some(vault) = a.get("vault").and_then(|v| v.as_str()) {
            cmd_args.push(format!("vault={}", vault));
        }
        
        run_obsidian_cmd(cmd_args)
    } else {
        json!({ "isError": true, "content": [{ "type": "text", "text": "Arguments requis manquants" }] })
    }
}

pub fn handle_obsidian_search(args: Option<&serde_json::Map<String, Value>>) -> Value {
    if let Some(a) = args {
        let mut cmd_args = vec!["search".to_string()];
        
        if let Some(query) = a.get("query").and_then(|v| v.as_str()) {
            cmd_args.push(format!("query={}", query));
        } else {
            return json!({ "isError": true, "content": [{ "type": "text", "text": "Le paramètre 'query' est obligatoire." }] });
        }
        
        if let Some(vault) = a.get("vault").and_then(|v| v.as_str()) {
            cmd_args.push(format!("vault={}", vault));
        }
        
        if let Some(format) = a.get("format").and_then(|v| v.as_str()) {
            cmd_args.push(format!("format={}", format));
        }
        
        run_obsidian_cmd(cmd_args)
    } else {
        json!({ "isError": true, "content": [{ "type": "text", "text": "Arguments requis manquants" }] })
    }
}
