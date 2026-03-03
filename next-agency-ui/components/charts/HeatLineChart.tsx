"use client";

/**
 * 熱度折線圖占位元件。
 * 可之後接 API /api/v1/trends/1h、/api/v1/trends/24h 的資料用 SVG 或圖表庫繪製。
 */
export function HeatLineChart() {
  return (
    <div className="flex h-48 items-center justify-center rounded border border-[#27272a] bg-[#09090b] text-[#71717a]">
      熱度折線圖（接 GET /api/v1/trends/1h、/api/v1/trends/24h）
    </div>
  );
}
