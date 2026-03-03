"use client";

import { useState } from "react";

const API_BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080";

export default function LeaksPage() {
  const [dmcaResult, setDmcaResult] = useState<{ email_body?: string } | null>(null);
  const [loading, setLoading] = useState(false);

  async function handleGenerateDMCA(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    setLoading(true);
    setDmcaResult(null);
    const form = e.currentTarget;
    const body = {
      copyright_holder: (form.elements.namedItem("copyright_holder") as HTMLInputElement).value,
      infringing_urls: ((form.elements.namedItem("infringing_urls") as HTMLTextAreaElement).value || "").split("\n").map((s) => s.trim()).filter(Boolean),
      original_work: (form.elements.namedItem("original_work") as HTMLInputElement).value,
      contact_email: (form.elements.namedItem("contact_email") as HTMLInputElement).value,
      contact_name: (form.elements.namedItem("contact_name") as HTMLInputElement).value,
    };
    try {
      const res = await fetch(`${API_BASE}/api/v1/dmca/generate`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(body),
      });
      const data = await res.json();
      setDmcaResult(data);
    } catch (err) {
      setDmcaResult({ email_body: String(err) });
    } finally {
      setLoading(false);
    }
  }

  return (
    <div>
      <h1 className="mb-2 text-2xl font-bold text-[#fafafa]">盜版外流監控與 DMCA 管理</h1>
      <p className="mb-6 text-[#a1a1aa]">外流資源列表與一鍵生成下架通知</p>

      <section className="mb-8 rounded-lg border border-[#27272a] bg-[#18181b] p-6">
        <h2 className="mb-4 font-semibold text-white">外流資源（phash 比對結果）</h2>
        <p className="text-sm text-[#71717a]">API: GET /api/v1/leaks — 資料由 Go 消費 Kafka 後填入</p>
      </section>

      <section className="rounded-lg border border-[#27272a] bg-[#18181b] p-6">
        <h2 className="mb-4 font-semibold text-white">生成 DMCA 下架通知</h2>
        <form onSubmit={handleGenerateDMCA} className="flex max-w-xl flex-col gap-3">
          <input name="copyright_holder" placeholder="版權持有者" required className="rounded border border-[#27272a] bg-[#09090b] px-3 py-2 text-white" />
          <textarea name="infringing_urls" placeholder="侵權 URL（一行一個）" rows={3} className="rounded border border-[#27272a] bg-[#09090b] px-3 py-2 text-white" />
          <input name="original_work" placeholder="原創作品說明" className="rounded border border-[#27272a] bg-[#09090b] px-3 py-2 text-white" />
          <input name="contact_email" type="email" placeholder="聯絡信箱" required className="rounded border border-[#27272a] bg-[#09090b] px-3 py-2 text-white" />
          <input name="contact_name" placeholder="聯絡人姓名" className="rounded border border-[#27272a] bg-[#09090b] px-3 py-2 text-white" />
          <button type="submit" disabled={loading} className="rounded bg-[#3b82f6] px-4 py-2 font-medium text-white disabled:opacity-50">
            {loading ? "生成中…" : "生成 Email 內文"}
          </button>
        </form>
        {dmcaResult?.email_body != null && (
          <pre className="mt-4 overflow-auto rounded border border-[#27272a] bg-[#09090b] p-4 text-sm text-[#a1a1aa]">
            {dmcaResult.email_body}
          </pre>
        )}
      </section>
    </div>
  );
}
