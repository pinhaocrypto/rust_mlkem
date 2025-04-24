impl Poly {
    /// Montgomery-domain 轉換
    #[inline]
    pub fn poly_tomont(&mut self) {
        poly_tomont(self);
    }

    /// Barrett reduction + signed→unsigned
    #[inline]
    pub fn poly_reduce(&mut self) {
        poly_reduce(self);
    }

    /// 多項式加法（就地）
    #[inline]
    pub fn poly_add(&mut self, other: &Poly) {
        poly_add(self, other);
    }

    /// 多項式減法（就地）
    #[inline]
    pub fn poly_sub(&mut self, other: &Poly) {
        poly_sub(self, other);
    }

    /// 預算 multiplication cache
    #[inline]
    pub fn poly_mulcache_compute(&mut self, cache: &mut PolyMulCache) {
        poly_mulcache_compute(cache, self);
    }

    /// 一個 block 的 NTT 蝶形
    #[inline]
    pub fn ntt_butterfly_block(&mut self, zeta: i16, start: usize, len: usize) {
        ntt_butterfly_block(self, zeta, start, len);
    }

    /// 一層 NTT
    #[inline]
    pub fn ntt_layer(&mut self, layer: usize) {
        ntt_layer(self, layer);
    }

    /// 完整 forward NTT
    #[inline]
    pub fn poly_ntt(&mut self) {
        poly_ntt(self);
    }

    /// 完整 inverse NTT + to Mont
    #[inline]
    pub fn poly_invntt_tomont(&mut self) {
        poly_invntt_tomont(self);
    }
}