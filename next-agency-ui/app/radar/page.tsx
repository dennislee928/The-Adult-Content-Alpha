import { HeatLineChart } from "@/components/charts/HeatLineChart";

export default function RadarPage() {
  return (
    <div>
      <h1 className="mb-2 text-2xl font-bold text-[#fafafa]">潛力新星挖掘</h1>
      <p className="mb-6 text-[#a1a1aa]">趨勢榜單與熱度折線圖</p>
      <section className="rounded-lg border border-[#27272a] bg-[#18181b] p-6">
        <h2 className="mb-4 font-semibold text-white">1h / 24h 熱度</h2>
        <HeatLineChart />
      </section>
    </div>
  );
}
