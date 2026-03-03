export default function HomePage() {
  return (
    <div>
      <h1 className="mb-2 text-2xl font-bold text-[#fafafa]">戰情室首頁</h1>
      <p className="mb-6 text-[#a1a1aa]">
        即時潛力創作者趨勢與外流資源警告。
      </p>
      <div className="grid gap-4 sm:grid-cols-3">
        <a
          href="/radar"
          className="rounded-lg border border-[#27272a] bg-[#18181b] p-5 transition hover:border-[#3f3f46]"
        >
          <h2 className="font-semibold text-white">潛力新星</h2>
          <p className="mt-1 text-sm text-[#71717a]">趨勢榜單與雷達圖</p>
        </a>
        <a
          href="/leaks"
          className="rounded-lg border border-[#27272a] bg-[#18181b] p-5 transition hover:border-[#3f3f46]"
        >
          <h2 className="font-semibold text-white">盜版外流</h2>
          <p className="mt-1 text-sm text-[#71717a]">監控與 DMCA 管理</p>
        </a>
        <a
          href="/creators"
          className="rounded-lg border border-[#27272a] bg-[#18181b] p-5 transition hover:border-[#3f3f46]"
        >
          <h2 className="font-semibold text-white">創作者</h2>
          <p className="mt-1 text-sm text-[#71717a]">旗下創作者資料庫</p>
        </a>
      </div>
    </div>
  );
}
