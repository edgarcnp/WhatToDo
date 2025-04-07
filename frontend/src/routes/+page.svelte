<script lang="ts">
    import type { PageData } from "./$types";

    type Todo = {
        id: number;
        description: string;
        status: boolean;
    };

    // Get todos from page load
    export let data: PageData;
    let todos = data.todos;
    let newDescription = "";

    // Create todo
    async function createTodo() {
        const res = await fetch("http://0.0.0.0:3000/create", {
            method: "POST",
            headers: { "Content-Type": "application/x-www-form-urlencoded" },
            body: new URLSearchParams({ description: newDescription }),
        });

        if (res.ok) {
            const newTodo: Todo = await res.json();
            todos = [...todos, newTodo];
            newDescription = "";
        }
    }

    // Update todo
    async function updateTodo(todo: Todo) {
        await fetch("http://0.0.0.0:3000/update", {
            method: "PUT",
            headers: { "Content-Type": "application/x-www-form-urlencoded" },
            body: new URLSearchParams({
                id: todo.id.toString(),
                description: todo.description,
                status: todo.status.toString(),
            }),
        });
    }

    // Delete todo
    async function deleteTodo(id: number) {
        await fetch(`http://0.0.0.0:3000/delete/${id}`, { method: "DELETE" });
        todos = todos.filter((todo: Todo) => todo.id !== id);
    }
</script>

<div class="container mx-auto mt-16">
    <h1 class="h1 text-center">Todos</h1>

    <div class="max-w-screen-md mx-auto">
        <div class="flex gap-4 my-8">
            <input
                class="input flex-1 p-4"
                name="description"
                type="text"
                placeholder="What needs to be done?"
                bind:value={newDescription}
                autocomplete="off"
                on:keypress={(e) => e.key === "Enter" && createTodo()}
            />
            <button class="btn variant-filled-primary" on:click={createTodo}
                >Add</button
            >
        </div>

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
                            on:click={() => updateTodo(todo)}>Update</button
                        >
                        <button
                            class="btn variant-filled-primary"
                            on:click={() => deleteTodo(todo.id)}>Delete</button
                        >
                    </div>
                </div>
            {/each}
        </div>
    </div>
</div>
