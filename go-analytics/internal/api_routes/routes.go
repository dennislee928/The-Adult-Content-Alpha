// Package api_routes 提供給 Next.js 的 REST API（趨勢、盜版比對、DMCA）。

package api_routes

import (
	"net/http"

	"github.com/adult-content-alpha/go-analytics/internal/dmca_generator"
	"github.com/gin-gonic/gin"
)

// Register 註冊所有 API 路由。
func Register(r *gin.Engine) {
	r.GET("/health", func(c *gin.Context) { c.JSON(http.StatusOK, gin.H{"status": "ok"}) })

	// 趨勢與雷達
	api := r.Group("/api/v1")
	api.GET("/trends/1h", getTrends1h)
	api.GET("/trends/24h", getTrends24h)
	api.GET("/radar", getRadar)

	// 盜版外流與 DMCA
	api.GET("/leaks", getLeaks)
	api.POST("/dmca/generate", postDMCAGenerate)
}

func getTrends1h(c *gin.Context) {
	// TODO: 從 trend_engine 讀取 1hr 熱度
	c.JSON(http.StatusOK, gin.H{"data": []interface{}{}, "window": "1h"})
}

func getTrends24h(c *gin.Context) {
	// TODO: 從 trend_engine 讀取 24hr 熱度
	c.JSON(http.StatusOK, gin.H{"data": []interface{}{}, "window": "24h"})
}

func getRadar(c *gin.Context) {
	// TODO: 潛力新星雷達圖資料
	c.JSON(http.StatusOK, gin.H{"data": []interface{}{}})
}

func getLeaks(c *gin.Context) {
	// TODO: 外流資源列表（phash 比對結果）
	c.JSON(http.StatusOK, gin.H{"data": []interface{}{}})
}

func postDMCAGenerate(c *gin.Context) {
	var body struct {
		CopyrightHolder string   `json:"copyright_holder" binding:"required"`
		InfringingURLs  []string `json:"infringing_urls" binding:"required"`
		OriginalWork    string   `json:"original_work"`
		ContactEmail    string   `json:"contact_email" binding:"required,email"`
		ContactName     string   `json:"contact_name"`
	}
	if err := c.ShouldBindJSON(&body); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	req := dmca_generator.TakedownRequest{
		CopyrightHolder: body.CopyrightHolder,
		InfringingURLs:  body.InfringingURLs,
		OriginalWork:    body.OriginalWork,
		ContactEmail:    body.ContactEmail,
		ContactName:     body.ContactName,
	}
	emailBody, err := dmca_generator.GenerateEmailBody(req)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	c.JSON(http.StatusOK, gin.H{"email_body": emailBody})
}
