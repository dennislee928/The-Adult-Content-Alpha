# 🍑 The Adult-Content Alpha (Digital Scout & Anti-Piracy)

## 專案簡介
專為成人內容創作者與經紀公司打造的 SaaS 後台。利用 Kafka 處理來自 X (Twitter)、Reddit (NSFW 版塊) 的海量貼文，實時計算特定創作者的「流量增長率」。同時監控各大論壇是否有未授權的付費內容外流，自動化產生版權檢舉（DMCA）報告。

## 核心技術棧
* **社群監聽:** Python, 各大平台 API 串接
* **訊息串流:** Apache Kafka, ksqlDB
* **資料庫:** PostgreSQL (時序資料)
* **後台視覺化:** Metabase 或 Grafana
* **影像指紋:** OpenCV / pHash (用於比對盜版圖片/影片)