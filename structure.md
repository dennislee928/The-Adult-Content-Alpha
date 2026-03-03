/adult-content-alpha
├── /social_ingestion         # 社群 API 抓取 (Producers)
│   ├── /twitter_stream       # 追蹤特定 hashtag 或連結
│   ├── /reddit_scraper       # 爬取 NSFW subreddits
│   └── /leak_forums          # 監控常見盜版資源網站
├── /kafka_ksqldb             # 實時資料流分析
│   ├── /ksql_queries         # 建立滑動窗口計算熱度 (SQL 語法)
│   └── docker-compose.yml    
├── /anti_piracy_engine       # 盜版防護模組 (Consumers)
│   ├── /media_hash_checker   # 計算圖片/影片的感知哈希 (pHash)
│   └── /dmca_generator       # 自動生成版權下架信件
├── /trend_analytics          # 趨勢計算模組
│   └── /growth_calculator    # 計算粉絲轉換率預估
├── /backend_api              # 提供 SaaS 儀表板資料的 Node.js API
└── /database                 # PostgreSQL schema 檔