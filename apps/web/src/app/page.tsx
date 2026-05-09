"use client";

import { useAuth } from "@/lib/auth-context";
import { useRouter } from "next/navigation";
import { useEffect } from "react";
import Link from "next/link";

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
    <main className="flex min-h-screen flex-col items-center justify-center bg-white text-gray-900 px-4">
      <div className="w-20 h-20 bg-blue-600 rounded-2xl flex items-center justify-center mb-8 shadow-2xl shadow-blue-200">
        <span className="text-white font-bold text-4xl">H</span>
      </div>
      <h1 className="text-5xl font-extrabold tracking-tight mb-4 text-center">
        Hemdaimail
      </h1>
      <p className="text-xl text-gray-500 mb-12 text-center max-w-lg">
        The modern, open-source webmail platform built for speed and security.
      </p>
      
      <div className="flex flex-col sm:flex-row gap-4 w-full max-w-xs sm:max-w-md">
        <Link 
          href="/login" 
          className="flex-1 bg-blue-600 hover:bg-blue-700 text-white font-bold py-4 px-8 rounded-2xl text-center transition-all shadow-lg shadow-blue-100"
        >
          Sign in
        </Link>
        <Link 
          href="/register" 
          className="flex-1 bg-gray-100 hover:bg-gray-200 text-gray-900 font-bold py-4 px-8 rounded-2xl text-center transition-all"
        >
          Create account
        </Link>
      </div>

      <footer className="absolute bottom-8 text-gray-400 text-sm">
        &copy; 2026 Hemdaimail Open Source Project
      </footer>
    </main>
  );
}
