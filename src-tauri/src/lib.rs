// ─────────────────────────────────────────────────────────────
// lib.rs — il "cervello" Rust di Plokk
// ─────────────────────────────────────────────────────────────
// Per ora c'è solo un comando: ping. Serve a verificare che
// la catena Svelte → Rust → Svelte funzioni. Lo sostituiremo
// con i comandi veri (leggere file, salvare shelf, ecc.)
// negli sprint successivi.
// ─────────────────────────────────────────────────────────────

/// Comando chiamabile da JS con `invoke("ping")`.
/// Non prende argomenti, restituisce una stringa fissa.
#[tauri::command]
fn ping() -> String {
    "pong".to_string()
}

/// Punto di partenza dell'app: registra i comandi e avvia Tauri.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Manteniamo opener per ora (è dichiarato in capabilities/default.json).
        .plugin(tauri_plugin_opener::init())
        // Registriamo `ping` tra i comandi visibili al frontend.
        // Se un comando non è qui, invoke() fallisce silenziosamente.
        .invoke_handler(tauri::generate_handler![ping])
        .run(tauri::generate_context!())
        .expect("errore durante l'avvio di Plokk");
}
