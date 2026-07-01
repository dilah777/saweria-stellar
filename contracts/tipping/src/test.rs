#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

#[test]
fn test_tipping_flow() {
    // 1. Inisialisasi Environment Soroban
    let env = Env::default();
    env.mock_all_auths(); // Matikan pengecekan auth untuk testing lokal

    // 2. Daftarkan kontrak ke dalam environment testing (menggunakan syntax SDK Anda)
    let contract_id = env.register(SaweriaContract, ());
    
    // 3. Buat "Client" untuk memanggil fungsi-fungsi kontrak
    let client = SaweriaContractClient::new(&env, &contract_id);

    // 4. Siapkan data palsu (mock data) untuk testing
    let sender = String::from_str(&env, "Guntur_Sender");
    let target = String::from_str(&env, "Creator_Target");
    let amount: u64 = 100_000_000; // Contoh nominal donasi
    let message = String::from_str(&env, "Semangat bikin konten!");

    // 5. TEST PERTAMA: Memanggil fungsi send_tip
    let response = client.send_tip(&sender, &target, &amount, &message);
    
    // Pastikan balasan dari kontrak sesuai dengan yang kita tulis di lib.rs
    assert_eq!(
        response,
        String::from_str(&env, "Tip successfully recorded on the blockchain!")
    );

    // 6. TEST KEDUA: Memanggil fungsi get_tips untuk mengecek data yang tersimpan
    let tips = client.get_tips();
    
    // Pastikan jumlah donasi yang tersimpan sekarang ada 1
    assert_eq!(tips.len(), 1);

    // Ambil donasi pertama dari daftar riwayat dan pastikan datanya cocok
    let recorded_tip = tips.get(0).unwrap();
    assert_eq!(recorded_tip.sender, sender);
    assert_eq!(recorded_tip.target, target);
    assert_eq!(recorded_tip.amount, amount);
    assert_eq!(recorded_tip.message, message);
}