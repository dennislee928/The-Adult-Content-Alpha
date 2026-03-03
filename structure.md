/adult-content-alpha
├── /rust-ingestion           # 數據抓取與影像運算 (Producer)
│   ├── Cargo.toml
│   ├── /src
│   │   ├── /scrapers         # X API, Reddit API 串接模組
│   │   ├── /vision           # pHash 運算模組 (整合 image-rs 或 OpenCV)
│   │   ├── rate_limiter.rs   # 嚴格的 API 請求頻率控制
│   │   └── kafka_sender.rs   # 將 metadata 與 pHash 打入 Kafka
├── /go-analytics             # 趨勢聚合與比對引擎 (Consumer & API)
│   ├── go.mod
│   ├── /cmd/server/main.go
│   ├── /internal
│   │   ├── /kafka_consumers  # 訂閱 `social-mentions` 與 `media-hashes`
│   │   ├── /trend_engine     # 滑動窗口演算法 (1hr, 24hr 熱度計算)
│   │   ├── /phash_matcher    # Hamming Distance 相似度比對演算法
│   │   ├── /dmca_generator   # 自動生成 PDF 或 Email 的下架通知
│   │   └── /api_routes       # 提供給 Next.js 的 GraphQL/REST API
├── /next-agency-ui           # 經紀公司戰情室 (Frontend)
│   ├── package.json
│   ├── /app
│   │   ├── /radar            # 潛力新星挖掘 (趨勢榜單)
│   │   ├── /leaks            # 盜版外流監控與 DMCA 管理
│   │   └── /creators         # 旗下創作者資料庫
│   └── /components/charts    # 雷達圖、熱度折線圖元件
└── docker-compose.yml        # Kafka, Zookeeper, PostgreSQL (含 pgvector), Redis