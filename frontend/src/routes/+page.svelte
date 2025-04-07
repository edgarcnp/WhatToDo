<script lang="ts">
    import type { PageData } from "./$types";
    import { goto } from "$app/navigation";

    type Todo = {
        id: number;
        description: string;
        status: boolean;
    };

    export let data: PageData;
    let todos = data.todos;

    async function deleteTodo(id: number) {
        await fetch(`http://0.0.0.0:8000/delete/${id}`, {
            method: "DELETE",
        });
        todos = todos.filter((todo: Todo) => todo.id !== id);
    }

    async function updateTodo(todo: Todo) {
        const form = new URLSearchParams();
        form.append("id", String(todo.id));
        form.append("description", todo.description);
        form.append("status", String(todo.status));

        await fetch("http://0.0.0.0:8000/update", {
            method: "PUT",
            headers: {
                "Content-Type": "application/x-www-form-urlencoded",
            },
            body: form.toString(),
        });

        // Optional: force page reload to refresh data
        await goto("/");
    }
</script>

<div class="container mx-auto mt-16">
    <h1 class="h1 text-center">Todos</h1>

    <div class="max-w-screen-md mx-auto">
        <form action="http://0.0.0.0:8000/create" method="POST">
            <input
                class="input p-4 my-8"
                name="description"
                type="text"
                placeholder="What needs to be done?"
                autocomplete="off"
            />
        </form>

        <div class="space-y-4">
            {#each todos as todo}
                <div
                    class="flex items-center justify-between p-4 bg-surface-800 rounded-lg gap-4"
                >
                    <input
                        class="checkbox"
                        type="checkbox"
                        bind:checked={todo.status}
                        on:change={() => updateTodo(todo)}
                    />
                    <input
                        class="input"
                        type="text"
                        bind:value={todo.description}
                        disabled={todo.status}
                    />

                    <div class="flex gap-2">
                        <button
                            class="btn variant-filled-secondary"
                            on:click={() => updateTodo(todo)}
                            type="button"
                        >
                            Update
                        </button>
                        <button
                            class="btn variant-filled-primary"
                            on:click={() => deleteTodo(todo.id)}
                            type="button"
                        >
                            Delete
                        </button>
                    </div>
                </div>
            {/each}
        </div>
    </div>
</div>
