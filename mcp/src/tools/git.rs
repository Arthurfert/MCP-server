use serde_json::{Value, json};
use std::process::Command;

pub fn handle_git_add(args: Option<&serde_json::Map<String, Value>>) -> Value {
    let custom_path = args
        .and_then(|a| a.get("path"))
        .and_then(|p| p.as_str())
        .unwrap_or("C:\\Users\\ferta\\Documents\\GitHub\\MCP-server");
    let files = args
        .and_then(|a| a.get("files"))
        .and_then(|p| p.as_str())
        .unwrap_or(".");

    let output = Command::new("git")
        .arg("add")
        .arg(files)
        .current_dir(custom_path)
        .output();

    match output {
        Ok(out) => {
            let mut result_str = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr_str = String::from_utf8_lossy(&out.stderr).to_string();
            if !stderr_str.is_empty() {
                result_str = format!("{}\nErreur/Warn:\n{}", result_str, stderr_str);
            }
            if result_str.trim().is_empty() {
                result_str = format!("Fichier(s) {} ajouté(s).", files);
            }
            json!({ "content": [{ "type": "text", "text": result_str }] })
        }
        Err(e) => {
            json!({ "isError": true, "content": [{ "type": "text", "text": format!("Erreur git add : {}", e) }] })
        }
    }
}

pub fn handle_git_commit(args: Option<&serde_json::Map<String, Value>>) -> Value {
    let custom_path = args
        .and_then(|a| a.get("path"))
        .and_then(|p| p.as_str())
        .unwrap_or("C:\\Users\\ferta\\Documents\\GitHub\\MCP-server");
    let message = args
        .and_then(|a| a.get("message"))
        .and_then(|p| p.as_str())
        .unwrap_or("Update");

    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .current_dir(custom_path)
        .output();

    match output {
        Ok(out) => {
            let mut result_str = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr_str = String::from_utf8_lossy(&out.stderr).to_string();
            if !stderr_str.is_empty() {
                result_str = format!("{}\nErreur/Warn:\n{}", result_str, stderr_str);
            }
            json!({ "content": [{ "type": "text", "text": result_str }] })
        }
        Err(e) => {
            json!({ "isError": true, "content": [{ "type": "text", "text": format!("Erreur git commit : {}", e) }] })
        }
    }
}

pub fn handle_git_status(args: Option<&serde_json::Map<String, Value>>) -> Value {
    let custom_path = args
        .and_then(|a| a.get("path"))
        .and_then(|p| p.as_str())
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
