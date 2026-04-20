#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Map, String, Symbol, Vec,
};

// ─────────────────────────────────────────────
// DATA TYPES
// ─────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum StatusLaporan {
    BelumDiproses,
    SedangDiproses,
    SudahDiperbaiki,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum KategoriLaporan {
    JalanRusak,
    Trotoar,
    TamanPublik,
    LampuLaluLintas,
    FasilitasUmumLain,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum Instansi {
    PUPR,
    Dishub,
    DLH,
    PemerintahDesa,
    PemerintahKecamatan,
    PemerintahKota,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Laporan {
    pub id: u64,
    pub pelapor: Address,
    pub nama_jalan: String,
    pub latitude: i64,  // koordinat × 1_000_000 (menghindari float)
    pub longitude: i64,
    pub kategori: KategoriLaporan,
    pub instansi: Instansi,
    pub status: StatusLaporan,
    pub jumlah_konfirmasi: u32,
    pub timestamp: u64,
    pub poin_diberikan: bool,
}

// ─────────────────────────────────────────────
// STORAGE KEYS
// ─────────────────────────────────────────────

const KEY_ADMIN: Symbol     = symbol_short!("ADMIN");
const KEY_COUNTER: Symbol   = symbol_short!("COUNTER");
const KEY_LAPORAN: Symbol   = symbol_short!("LAPORAN");
const KEY_POIN: Symbol      = symbol_short!("POIN");
const KEY_PEMRINTAH: Symbol = symbol_short!("PEMRINTAH");
const KEY_KONFIRM: Symbol   = symbol_short!("KONFIRM");

// ─────────────────────────────────────────────
// CONTRACT
// ─────────────────────────────────────────────

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {

    // ── INISIALISASI ──────────────────────────

    /// Inisialisasi kontrak, hanya bisa dipanggil sekali.
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&KEY_ADMIN) {
            panic!("Kontrak sudah diinisialisasi");
        }
        env.storage().instance().set(&KEY_ADMIN, &admin);
        env.storage().instance().set(&KEY_COUNTER, &0u64);
        env.storage().instance().set(&KEY_LAPORAN, &Map::<u64, Laporan>::new(&env));
        env.storage().instance().set(&KEY_POIN, &Map::<Address, u64>::new(&env));
        env.storage().instance().set(&KEY_PEMRINTAH, &Vec::<Address>::new(&env));
        env.storage().instance().set(&KEY_KONFIRM, &Map::<u64, Vec<Address>>::new(&env));
    }

    // ── MANAJEMEN PEMERINTAH (admin only) ─────

    /// Daftarkan alamat sebagai akun pemerintah.
    pub fn daftar_pemerintah(env: Env, caller: Address, akun: Address) {
        caller.require_auth();
        Self::require_admin(&env, &caller);

        let mut daftar: Vec<Address> = env.storage().instance().get(&KEY_PEMRINTAH).unwrap();
        daftar.push_back(akun);
        env.storage().instance().set(&KEY_PEMRINTAH, &daftar);
    }

    // ── PELAPORAN OLEH MASYARAKAT ─────────────

    /// Buat laporan kerusakan baru.
    /// `lat` dan `lon` adalah koordinat GPS × 1_000_000.
    /// Mengembalikan ID laporan yang baru dibuat.
    pub fn buat_laporan(
        env: Env,
        pelapor: Address,
        nama_jalan: String,
        lat: i64,
        lon: i64,
        kategori: KategoriLaporan,
    ) -> u64 {
        pelapor.require_auth();

        let instansi = Self::tentukan_instansi(&kategori);

        let mut counter: u64 = env.storage().instance().get(&KEY_COUNTER).unwrap();
        counter += 1;
        env.storage().instance().set(&KEY_COUNTER, &counter);

        let laporan = Laporan {
            id: counter,
            pelapor: pelapor.clone(),
            nama_jalan,
            latitude: lat,
            longitude: lon,
            kategori,
            instansi,
            status: StatusLaporan::BelumDiproses,
            jumlah_konfirmasi: 1,
            timestamp: env.ledger().timestamp(),
            poin_diberikan: false,
        };

        let mut semua: Map<u64, Laporan> = env.storage().instance().get(&KEY_LAPORAN).unwrap();
        semua.set(counter, laporan);
        env.storage().instance().set(&KEY_LAPORAN, &semua);

        // Catat pelapor sebagai konfirmator pertama
        let mut konfirm: Map<u64, Vec<Address>> = env.storage().instance().get(&KEY_KONFIRM).unwrap();
        let mut list = Vec::new(&env);
        list.push_back(pelapor.clone());
        konfirm.set(counter, list);
        env.storage().instance().set(&KEY_KONFIRM, &konfirm);

        Self::tambah_poin(&env, &pelapor, 10);

        counter
    }

    /// Warga lain mengkonfirmasi laporan yang sudah ada.
    /// Satu address hanya bisa konfirmasi satu laporan sekali.
    pub fn konfirmasi_laporan(env: Env, pelapor: Address, id: u64) {
        pelapor.require_auth();

        let mut konfirm: Map<u64, Vec<Address>> = env.storage().instance().get(&KEY_KONFIRM).unwrap();
        let mut list = konfirm.get(id).unwrap_or(Vec::new(&env));

        for addr in list.iter() {
            if addr == pelapor {
                panic!("Anda sudah mengkonfirmasi laporan ini");
            }
        }

        list.push_back(pelapor.clone());
        konfirm.set(id, list);
        env.storage().instance().set(&KEY_KONFIRM, &konfirm);

        let mut semua: Map<u64, Laporan> = env.storage().instance().get(&KEY_LAPORAN).unwrap();
        let mut laporan = semua.get(id).expect("Laporan tidak ditemukan");
        laporan.jumlah_konfirmasi += 1;
        semua.set(id, laporan);
        env.storage().instance().set(&KEY_LAPORAN, &semua);

        Self::tambah_poin(&env, &pelapor, 5);
    }

    // ── PENGELOLAAN OLEH PEMERINTAH ───────────

    /// Pemerintah mengubah status penanganan laporan.
    /// Jika status menjadi SudahDiperbaiki, pelapor mendapat bonus poin.
    pub fn update_status(env: Env, pemerintah: Address, id: u64, status_baru: StatusLaporan) {
        pemerintah.require_auth();
        Self::require_pemerintah(&env, &pemerintah);

        let mut semua: Map<u64, Laporan> = env.storage().instance().get(&KEY_LAPORAN).unwrap();
        let mut laporan = semua.get(id).expect("Laporan tidak ditemukan");

        let selesai = status_baru == StatusLaporan::SudahDiperbaiki;
        laporan.status = status_baru;

        if selesai && !laporan.poin_diberikan {
            Self::tambah_poin(&env, &laporan.pelapor, 50);
            laporan.poin_diberikan = true;
        }

        semua.set(id, laporan);
        env.storage().instance().set(&KEY_LAPORAN, &semua);
    }

    // ── QUERY / READ ──────────────────────────

    /// Ambil data satu laporan berdasarkan ID.
    pub fn get_laporan(env: Env, id: u64) -> Laporan {
        let semua: Map<u64, Laporan> = env.storage().instance().get(&KEY_LAPORAN).unwrap();
        semua.get(id).expect("Laporan tidak ditemukan")
    }

    /// Ambil semua laporan (untuk dashboard pemerintah).
    pub fn get_semua_laporan(env: Env) -> Map<u64, Laporan> {
        env.storage().instance().get(&KEY_LAPORAN).unwrap()
    }

    /// Ambil daftar ID laporan diurutkan berdasarkan konfirmasi terbanyak (prioritas tertinggi).
    pub fn get_prioritas(env: Env) -> Vec<u64> {
        let semua: Map<u64, Laporan> = env.storage().instance().get(&KEY_LAPORAN).unwrap();
        let total: u64 = env.storage().instance().get(&KEY_COUNTER).unwrap();

        let mut ids: Vec<u64> = Vec::new(&env);
        let mut i = 1u64;
        while i <= total {
            if semua.contains_key(i) {
                ids.push_back(i);
            }
            i += 1;
        }

        // Bubble sort descending berdasarkan jumlah_konfirmasi
        let len = ids.len();
        let mut j = 0u32;
        while j < len {
            let mut k = 0u32;
            while k < len - j - 1 {
                let id_a = ids.get(k).unwrap();
                let id_b = ids.get(k + 1).unwrap();
                let lap_a = semua.get(id_a).unwrap();
                let lap_b = semua.get(id_b).unwrap();
                if lap_a.jumlah_konfirmasi < lap_b.jumlah_konfirmasi {
                    ids.set(k, id_b);
                    ids.set(k + 1, id_a);
                }
                k += 1;
            }
            j += 1;
        }

        ids
    }

    /// Ambil total poin milik pengguna tertentu.
    pub fn get_poin(env: Env, pengguna: Address) -> u64 {
        let poin_map: Map<Address, u64> = env.storage().instance().get(&KEY_POIN).unwrap();
        poin_map.get(pengguna).unwrap_or(0)
    }

    /// Ambil total jumlah laporan yang pernah dibuat.
    pub fn get_total_laporan(env: Env) -> u64 {
        env.storage().instance().get(&KEY_COUNTER).unwrap()
    }

    // ── INTERNAL HELPERS ──────────────────────

    fn require_admin(env: &Env, caller: &Address) {
        let admin: Address = env.storage().instance().get(&KEY_ADMIN).unwrap();
        if *caller != admin {
            panic!("Hanya admin yang boleh melakukan aksi ini");
        }
    }

    fn require_pemerintah(env: &Env, caller: &Address) {
        let daftar: Vec<Address> = env.storage().instance().get(&KEY_PEMRINTAH).unwrap();
        for addr in daftar.iter() {
            if addr == *caller {
                return;
            }
        }
        panic!("Hanya akun pemerintah yang boleh melakukan aksi ini");
    }

    fn tambah_poin(env: &Env, pengguna: &Address, jumlah: u64) {
        let mut poin_map: Map<Address, u64> = env.storage().instance().get(&KEY_POIN).unwrap();
        let lama = poin_map.get(pengguna.clone()).unwrap_or(0);
        poin_map.set(pengguna.clone(), lama + jumlah);
        env.storage().instance().set(&KEY_POIN, &poin_map);
    }

    fn tentukan_instansi(kategori: &KategoriLaporan) -> Instansi {
        match kategori {
            KategoriLaporan::JalanRusak        => Instansi::PUPR,
            KategoriLaporan::Trotoar           => Instansi::Dishub,
            KategoriLaporan::LampuLaluLintas   => Instansi::Dishub,
            KategoriLaporan::TamanPublik       => Instansi::DLH,
            KategoriLaporan::FasilitasUmumLain => Instansi::PemerintahKota,
        }
    }
}

mod test;