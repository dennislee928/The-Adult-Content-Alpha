import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Agency 戰情室",
  description: "潛力新星趨勢與盜版外流監控",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="zh-Hant" className="dark">
      <body className="min-h-screen bg-[#0d0d0f] text-[#e4e4e7] antialiased">
        <nav className="border-b border-[#27272a] bg-[#18181b]/80 backdrop-blur">
          <div className="mx-auto flex h-14 max-w-7xl items-center gap-6 px-4">
            <a href="/" className="font-semibold text-[#fafafa]">
              Agency 戰情室
            </a>
            <a href="/radar" className="text-[#a1a1aa] hover:text-white">
              潛力新星
            </a>
            <a href="/leaks" className="text-[#a1a1aa] hover:text-white">
              盜版外流
            </a>
            <a href="/creators" className="text-[#a1a1aa] hover:text-white">
              創作者
            </a>
          </div>
        </nav>
        <main className="mx-auto max-w-7xl px-4 py-8">{children}</main>
      </body>
    </html>
  );
}
