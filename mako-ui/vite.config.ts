import { resolve } from "node:path";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

// BASE_PATH lets a hosted build live under a sub-path (e.g. "/ewn/" on the
// shared Uberspace host). Defaults to "/" for local dev and standalone builds.
const base = process.env["BASE_PATH"] ?? "/";

export default defineConfig({
	base,
	plugins: [react(), tailwindcss()],
	resolve: {
		alias: {
			"@": resolve(__dirname, "./src"),
		},
	},
	server: {
		proxy: {
			"/api": "http://localhost:3001",
		},
	},
});
