// Package dmca_generator 自動生成 PDF 或 Email 的下架通知（DMCA Takedown）。

package dmca_generator

import (
	"bytes"
	"fmt"
	"text/template"
	"time"
)

// TakedownRequest 下架請求內容。
type TakedownRequest struct {
	CopyrightHolder string
	InfringingURLs  []string
	OriginalWork    string
	ContactEmail    string
	ContactName     string
	Date            string
}

// GenerateEmailBody 產生純文字 Email 內文，可轉成 PDF 或直接寄出。
func GenerateEmailBody(req TakedownRequest) (string, error) {
	if req.Date == "" {
		req.Date = time.Now().UTC().Format(time.RFC3339)
	}
	tpl := template.Must(template.New("dmca").Parse(dmcaEmailTemplate))
	var buf bytes.Buffer
	if err := tpl.Execute(&buf, req); err != nil {
		return "", fmt.Errorf("template: %w", err)
	}
	return buf.String(), nil
}

const dmcaEmailTemplate = `DMCA Takedown Notice

Date: {{.Date}}

I am {{.ContactName}}, acting on behalf of the copyright holder: {{.CopyrightHolder}}
Contact: {{.ContactEmail}}

I have a good faith belief that the following material is infringing:
Original work: {{.OriginalWork}}

Infringing URLs:
{{range .InfringingURLs}}
- {{.}}
{{end}}

I declare under penalty of perjury that the information in this notice is accurate.

Signature: {{.ContactName}}
`
