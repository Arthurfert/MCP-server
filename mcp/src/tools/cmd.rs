use serde_json::{json, Value};
use std::process::Command;

pub fn handle_run_command(args: Option<&serde_json::Map<String, Value>>) -> Value {
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