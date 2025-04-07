import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
    const res = await fetch(`${import.meta.env.VITE_BACKEND_URL}`);

    if (!res.ok) {
        throw new Error("Failed to fetch todos");
    }

    return {
        todos: await res.json()
    };
};
