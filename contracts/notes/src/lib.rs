#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data untuk menyimpan tugas (Task)
#[contracttype]
#[derive(Clone, Debug)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub is_completed: bool, // Menandakan apakah tugas sudah selesai
}

// Storage key untuk data tugas
const TASK_DATA: Symbol = symbol_short!("TASK_DATA");

#[contract]
pub struct TaskContract;

#[contractimpl]
impl TaskContract {
    
    // 1. Fungsi untuk mendapatkan semua daftar tugas
    pub fn get_tasks(env: Env) -> Vec<Task> {
        return env.storage().instance().get(&TASK_DATA).unwrap_or(Vec::new(&env));
    }

    // 2. Fungsi untuk membuat tugas baru
    pub fn create_task(env: Env, title: String, description: String) -> String {
        let mut tasks: Vec<Task> = env.storage().instance().get(&TASK_DATA).unwrap_or(Vec::new(&env));
        
        let task = Task {
            id: env.prng().gen::<u64>(),
            title: title,
            description: description,
            is_completed: false, // Default saat dibuat: belum selesai
        };
        
        tasks.push_back(task);
        env.storage().instance().set(&TASK_DATA, &tasks);
        
        return String::from_str(&env, "Tugas berhasil ditambahkan ke daftar");
    }

    // 3. FUNGSI BARU: Menandai tugas sebagai selesai (tanpa dihapus)
    pub fn complete_task(env: Env, id: u64) -> String {
        let mut tasks: Vec<Task> = env.storage().instance().get(&TASK_DATA).unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {
            let mut task = tasks.get(i).unwrap();
            
            if task.id == id {
                task.is_completed = true; // Ubah status menjadi selesai
                
                // Gunakan fungsi .set() untuk menimpa elemen pada index tersebut di dalam Vec
                tasks.set(i, task); 
                
                env.storage().instance().set(&TASK_DATA, &tasks);
                return String::from_str(&env, "Status tugas diupdate: Selesai!");
            }
        }

        return String::from_str(&env, "Tugas tidak ditemukan");
    }

    // 4. Fungsi untuk menghapus tugas berdasarkan id
    pub fn delete_task(env: Env, id: u64) -> String {
        let mut tasks: Vec<Task> = env.storage().instance().get(&TASK_DATA).unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {
            if tasks.get(i).unwrap().id == id {
                tasks.remove(i);
                env.storage().instance().set(&TASK_DATA, &tasks);
                return String::from_str(&env, "Berhasil menghapus tugas dari sistem");
            }
        }

        return String::from_str(&env, "Tugas tidak ditemukan");
    }
} 

mod test;










/* --- CONTOH SCRIPT ---

pub fn get_notes(env: Env) -> Vec<Note> {
    // 1. ambil data notes dari storage
    return env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
}

// Fungsi untuk membuat note baru
pub fn create_note(env: Env, title: String, content: String) -> String {
    // 1. ambil data notes dari storage
    let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
    
    // 2. Buat object note baru
    let note = Note {
        id: env.prng().gen::<u64>(),
        title: title,
        content: content,
    };
    
    // 3. tambahkan note baru ke notes lama
    notes.push_back(note);
    
    // 4. simpan notes ke storage
    env.storage().instance().set(&NOTE_DATA, &notes);
    
    return String::from_str(&env, "Notes berhasil ditambahkan");
}

// Fungsi untuk menghapus notes berdasarkan id
pub fn delete_note(env: Env, id: u64) -> String {
    // 1. ambil data notes dari storage 
    let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));

    // 2. cari index note yang akan dihapus menggunakan perulangan
    for i in 0..notes.len() {
        if notes.get(i).unwrap().id == id {
            notes.remove(i);

            env.storage().instance().set(&NOTE_DATA, &notes);
            return String::from_str(&env, "Berhasil hapus notes");
        }
    }

    return String::from_str(&env, "Notes tidak ditemukan")
}


*/