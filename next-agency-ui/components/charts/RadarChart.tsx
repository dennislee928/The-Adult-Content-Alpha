"use client";

/**
 * 雷達圖占位元件。
 * 可之後接 GET /api/v1/radar 的資料繪製多維度潛力新星雷達圖。
 */
export function RadarChart() {
  return (
    <div className="flex h-64 items-center justify-center rounded border border-[#27272a] bg-[#09090b] text-[#71717a]">
      雷達圖（接 GET /api/v1/radar）
    </div>
  );
}
