# 🍑 The Adult-Content Alpha (Rust + Go + Next.js)

## 專案簡介
專為成人內容創作者與經紀公司打造的 SaaS 戰情室。
本系統利用 Rust 進行極速的社群網路監聽與影像感知哈希 (pHash) 運算，透過 Kafka 將海量數據流交由 Go 進行實時熱度聚合與盜版比對，最後在 Next.js 前端呈現具備商業價值的「新星趨勢榜」與「一鍵 DMCA 檢舉」功能。

## 系統架構
* **Ingestion & Computer Vision (Rust):** 高併發抓取 X (Twitter) 與 Reddit NSFW 貼文。當偵測到多媒體內容時，直接在記憶體內計算 pHash (Perceptual Hash)，並將文字情緒與哈希值打入 Kafka。
* **Stream Analytics & Matching (Go):** 負責消費 Kafka 串流，利用 Goroutines 處理「時間滑動窗口 (Sliding Window)」計算創作者漲粉斜率。同時，將 Rust 傳來的 pHash 與資料庫內的「受保護版權物」進行快速比對 (Hamming Distance)。
* **Agency Dashboard (Next.js):** 提供給 B2B 客戶的高質感深色系儀表板，即時顯示潛力創作者雷達圖與外流資源警告。
* **Message Broker & DB:** Apache Kafka, PostgreSQL (儲存創作者資料與哈希特徵), Redis (快取熱門排行榜)。

## ⚠️ 合規與免責聲明
本專案為數據分析與版權保護工具，**絕對不**儲存、託管或傳播任何未授權的成人多媒體檔案本身，資料庫僅保留數學哈希值 (Hashes) 與公開社群數據。