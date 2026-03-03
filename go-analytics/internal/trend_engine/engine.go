// Package trend_engine 滑動窗口演算法：1hr、24hr 熱度計算。

package trend_engine

import (
	"sync"
	"time"
)

// Bucket 單一時間桶的計數。
type Bucket struct {
	Count int
	At    time.Time
}

// SlidingWindow 滑動時間窗口，用於計算近期熱度。
type SlidingWindow struct {
	mu      sync.RWMutex
	buckets []Bucket
	window  time.Duration
}

// NewSlidingWindow 建立一個滑動窗口，例如 1hr 或 24hr。
func NewSlidingWindow(window time.Duration) *SlidingWindow {
	return &SlidingWindow{
		buckets: nil,
		window:  window,
	}
}

// Add 在當前時間加一筆計數。
func (s *SlidingWindow) Add() {
	s.mu.Lock()
	defer s.mu.Unlock()
	now := time.Now().UTC()
	s.prune(now)
	// 合併到最近一個 bucket（同一分鐘）
	if len(s.buckets) > 0 {
		last := &s.buckets[len(s.buckets)-1]
		if now.Sub(last.At) < time.Minute {
			last.Count++
			return
		}
	}
	s.buckets = append(s.buckets, Bucket{Count: 1, At: now})
}

func (s *SlidingWindow) prune(now time.Time) {
	cut := now.Add(-s.window)
	i := 0
	for i < len(s.buckets) && s.buckets[i].At.Before(cut) {
		i++
	}
	if i > 0 {
		s.buckets = s.buckets[i:]
	}
}

// Total 回傳窗口內總計數。
func (s *SlidingWindow) Total() int {
	s.mu.RLock()
	defer s.mu.RUnlock()
	now := time.Now().UTC()
	var total int
	cut := now.Add(-s.window)
	for _, b := range s.buckets {
		if b.At.After(cut) {
			total += b.Count
		}
	}
	return total
}
