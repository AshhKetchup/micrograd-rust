struct BPETokeniser {
    VocabCount: u32,
}

impl BPETokeniser {
    pub fn init(cnt: int) {
        VocabCount = cnt;
    }
    pub fn train_bpe(text: String, num_merges: u32) {
        let mut tok_2_id: HashMap<String, u32> = HashMap::new();
        let mut id_2_tok: Vec<String> = Vec::new();
        let mut merge_rules: HashMap<(u32, u32), u64> = HashMap::new();
        let mut pair_counts: HashMap<(u32, u32), u64> = HashMap::new();
        println!("[TOKENISATION] BPE training started...");
        let mut cnt = 0;
        for ch in text {
            if (!tok_2_id.contains_key(&ch)) {
                tok_2_id[ch] = cnt;
                cnt += 1;
            }
        }
    }
}
