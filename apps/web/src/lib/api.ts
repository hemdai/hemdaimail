const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3000";

async function request(endpoint: string, options: RequestInit = {}) {
  const token = typeof window !== "undefined" ? localStorage.getItem("auth_token") : null;

  const headers = {
    "Content-Type": "application/json",
    ...(token ? { Authorization: `Bearer ${token}` } : {}),
    ...options.headers,
  };

  const response = await fetch(`${API_BASE_URL}${endpoint}`, {
    ...options,
    headers,
  });

  if (!response.ok) {
    const error = await response.json().catch(() => ({ message: "An error occurred" }));
    throw new Error(error.message || response.statusText);
  }

  return response.json();
}

export const api = {
  auth: {
    login: (data: any) => request("/auth/login", { method: "POST", body: JSON.stringify(data) }),
    register: (data: any) => request("/auth/register", { method: "POST", body: JSON.stringify(data) }),
  },
  mail: {
    listMailboxes: () => request("/mailboxes"),
    listMessages: (mailboxId: string, params: any = {}) => {
      const query = new URLSearchParams(params).toString();
      return request(`/mailboxes/${mailboxId}/messages${query ? `?${query}` : ""}`);
    },
    sendEmail: (data: any) => request("/mail/send", { method: "POST", body: JSON.stringify(data) }),
  },
};
