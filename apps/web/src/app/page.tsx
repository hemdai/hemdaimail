"use client";

import { useAuth } from "@/lib/auth-context";
import { useRouter } from "next/navigation";
import { useEffect } from "react";
import Link from "next/link";
import { Mail, Shield, Zap, Search } from "lucide-react";

export default function Home() {
  const { token, isLoading } = useAuth();
  const router = useRouter();

  useEffect(() => {
    if (!isLoading && token) {
      router.push("/inbox");
    }
  }, [token, isLoading, router]);

  if (isLoading) return null;

  return (
    <div className="min-h-screen bg-white text-gray-900">
      {/* Navbar */}
      <nav className="flex items-center justify-between px-8 py-6">
        <div className="flex items-center gap-2">
          <div className="w-10 h-10 bg-blue-600 rounded-lg flex items-center justify-center">
            <Mail className="w-6 h-6 text-white" />
          </div>
          <span className="text-2xl font-bold tracking-tight">Hemdaimail</span>
        </div>
        <div className="flex items-center gap-4">
          <Link href="/login" className="text-sm font-medium text-gray-600 hover:text-blue-600">
            Sign in
          </Link>
          <Link href="/register" className="text-sm font-medium bg-blue-600 text-white px-5 py-2.5 rounded-full hover:bg-blue-700 transition">
            Create account
          </Link>
        </div>
      </nav>

      {/* Hero Section */}
      <main className="container mx-auto px-8 py-16 md:py-24">
        <div className="max-w-3xl">
          <h1 className="text-6xl md:text-7xl font-extrabold tracking-tight mb-8 leading-[1.1]">
            Your email, <span className="text-blue-600">reimagined</span> for speed.
          </h1>
          <p className="text-xl text-gray-600 mb-10 max-w-2xl leading-relaxed">
            Hemdaimail brings the productivity of modern webmail to your fingertips. Open-source, secure, and built for the way you work.
          </p>
          <div className="flex flex-col sm:flex-row gap-4">
            <Link 
              href="/register" 
              className="bg-blue-600 hover:bg-blue-700 text-white font-semibold py-4 px-10 rounded-full text-lg transition-all shadow-lg hover:shadow-blue-200"
            >
              Get started for free
            </Link>
          </div>
        </div>

        {/* Feature Grid */}
        <div className="grid md:grid-cols-3 gap-12 mt-24">
          {[
            { icon: Zap, title: "Blazing Fast", desc: "Built with Rust and real-time WebSockets for near-instant interaction." },
            { icon: Shield, title: "Privacy First", desc: "Your data stays yours. Open-source, auditable, and secure by design." },
            { icon: Search, title: "Powerful Search", desc: "Find anything instantly with high-performance Meilisearch integration." },
          ].map((feature, i) => (
            <div key={i} className="flex flex-col gap-4">
              <div className="w-12 h-12 bg-blue-50 rounded-2xl flex items-center justify-center text-blue-600">
                <feature.icon className="w-6 h-6" />
              </div>
              <h3 className="text-xl font-semibold">{feature.title}</h3>
              <p className="text-gray-500 leading-relaxed">{feature.desc}</p>
            </div>
          ))}
        </div>
      </main>

      {/* Footer */}
      <footer className="px-8 py-8 border-t border-gray-100 text-center text-gray-400 text-sm">
        &copy; 2026 Hemdaimail Open Source Project
      </footer>
    </div>
  );
}
