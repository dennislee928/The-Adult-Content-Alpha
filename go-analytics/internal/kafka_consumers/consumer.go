// Package kafka_consumers 訂閱 social-mentions 與 media-hashes，寫入記憶體/DB 供趨勢與比對使用。

package kafka_consumers

import (
	"context"
	"encoding/json"
	"log"

	"github.com/IBM/sarama"
)

const (
	TopicSocialMentions = "social-mentions"
	TopicMediaHashes   = "media-hashes"
)

// SocialMention 與 Rust 端 SocialMentionEvent 對應。
type SocialMention struct {
	Platform      string  `json:"platform"`
	PostID        string  `json:"post_id"`
	AuthorID      string  `json:"author_id"`
	AuthorHandle  string  `json:"author_handle"`
	ContentText   string  `json:"content_text"`
	SentimentScore *float32 `json:"sentiment_score"`
	CreatedAtUTC  string  `json:"created_at_utc"`
}

// MediaHash 與 Rust 端 MediaHashEvent 對應。
type MediaHash struct {
	Platform    string `json:"platform"`
	PostID      string `json:"post_id"`
	MediaURL    string `json:"media_url"`
	PhashHex    string `json:"phash_hex"`
	CreatedAtUTC string `json:"created_at_utc"`
}

// RunConsumers 啟動 social-mentions 與 media-hashes 的 consumer。
func RunConsumers(ctx context.Context, brokers string) {
	cfg := sarama.NewConfig()
	cfg.Consumer.Return.Errors = true
	cfg.Version = sarama.V3_0_0_0

	client, err := sarama.NewConsumer([]string{brokers}, cfg)
	if err != nil {
		log.Printf("kafka consumer: %v", err)
		return
	}
	defer client.Close()

	topics := []string{TopicSocialMentions, TopicMediaHashes}
	for _, topic := range topics {
		pc, err := client.ConsumePartition(topic, 0, sarama.OffsetNewest)
		if err != nil {
			log.Printf("consume %s: %v", topic, err)
			continue
		}
		go consumePartition(ctx, topic, pc)
	}

	<-ctx.Done()
}

func consumePartition(ctx context.Context, topic string, pc sarama.PartitionConsumer) {
	defer pc.Close()
	for {
		select {
		case <-ctx.Done():
			return
		case msg := <-pc.Messages():
			handleMessage(topic, msg.Value)
		case err := <-pc.Errors():
			if err != nil {
				log.Printf("consumer %s err: %v", topic, err.Err)
			}
		}
	}
}

func handleMessage(topic string, raw []byte) {
	switch topic {
	case TopicSocialMentions:
		var m SocialMention
		if err := json.Unmarshal(raw, &m); err != nil {
			log.Printf("unmarshal social-mention: %v", err)
			return
		}
		// TODO: 寫入 trend_engine 滑動窗口或 DB
		_ = m
	case TopicMediaHashes:
		var m MediaHash
		if err := json.Unmarshal(raw, &m); err != nil {
			log.Printf("unmarshal media-hash: %v", err)
			return
		}
		// TODO: 寫入 phash_matcher 或 DB 供比對
		_ = m
	}
}
