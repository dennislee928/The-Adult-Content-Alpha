// Package phash_matcher 使用 Hamming Distance 比對兩筆 pHash 相似度。

package phash_matcher

import (
	"encoding/hex"
	"errors"
)

// HammingDistance 計算兩組相同長度的 bytes 的漢明距離（相異 bit 數）。
func HammingDistance(a, b []byte) (int, error) {
	if len(a) != len(b) {
		return 0, errors.New("length mismatch")
	}
	var d int
	for i := range a {
		d += bitsSet(a[i] ^ b[i])
	}
	return d, nil
}

func bitsSet(x byte) int {
	var n int
	for x != 0 {
		n += int(x & 1)
		x >>= 1
	}
	return n
}

// PhashHexToBytes 將 Rust 端產出的 hex 字串解成 bytes（用於比對）。
func PhashHexToBytes(hexStr string) ([]byte, error) {
	return hex.DecodeString(hexStr)
}

// Similarity 回傳 0~1，1 表示完全相同。threshold 為漢明距離上限，超過視為不相似。
func Similarity(phashA, phashB string, maxBits int) (float64, error) {
	a, err := PhashHexToBytes(phashA)
	if err != nil {
		return 0, err
	}
	b, err := PhashHexToBytes(phashB)
	if err != nil {
		return 0, err
	}
	dist, err := HammingDistance(a, b)
	if err != nil {
		return 0, err
	}
	totalBits := len(a) * 8
	if dist <= maxBits {
		return 1.0 - float64(dist)/float64(totalBits), nil
	}
	return 0, nil
}
