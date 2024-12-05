<template>
    <main class="w-full max-w-7xl mx-auto flex flex-col items-center gap-4 py-5">
        <h1 class="text-5xl mb-5">ICP Bootcamp Blog</h1>
        <div>
            <form
                action="#"
                @submit.prevent="handleSubmit"
                class="flex flex-col border border-black p-5 gap-4 w-[700px] mb-10"
            >
                <h2 class="text-3xl font-bold">Add new blog</h2>
                <div>
                    <label for="title" class="block font-medium mb-1">Title:</label>
                    <input
                        id="title"
                        type="text"
                        v-model="formData.title"
                        class="w-full border border-black px-4 py-2"
                    />
                </div>
                <div>
                    <label for="content" class="block font-medium mb-1">Content:</label>
                    <textarea
                        id="content"
                        type="text"
                        v-model="formData.content"
                        class="w-full border border-black px-4 py-2"
                    ></textarea>
                </div>
                <div>
                    <label for="tags" class="block font-medium mb-1">Tags:</label>
                    <input
                        id="tags"
                        type="text"
                        v-model="formData.tags"
                        class="w-full border border-black px-4 py-2"
                    />
                </div>
                <button
                    type="submit"
                    class="bg-sky-700 text-white rounded-md py-2 px-4 hover:bg-sky-600 ease-in duration-300"
                >
                    Add Blog
                </button>
            </form>
        </div>

        <div class="w-[700px]">
            <h2 class="text-2xl font-bold mb-4">Our blogs:</h2>
            <div
                v-for="(blog, key) in blogs"
                :key="key"
                class="border border-black p-4 mb-10"
            >
                <p class="mb-2"><span class="font-semibold">Title:</span> {{ blog.title }}</p>
                <p class="mb-2"><span class="font-semibold">Content:</span> {{ blog.content }}</p>
                <p class="mb-2"><span class="font-semibold">Tags:</span> {{ blog.tags.toString() }}</p>
                <p class="mb-2"><span class="font-semibold">Date:</span> {{ blog.date }}</p>
            </div>
        </div>
    </main>
</template>

<script setup lang="ts">
    import { ref, reactive } from "vue";
    import { blog_backend } from "../../declarations/blog_backend";

    interface FormData {
        title: string;
        content: string;
        tags: string;
    }

    interface Blog {
        title: string;
        date: string;
        content: string;
        tags: Array<string>;
    }

    const blogs = ref<Blog[] | null>(null);

    const formData = reactive<FormData>({
        title: "",
        content: "",
        tags: ""
    });

    const handleSubmit = async () => {
        const response = await blog_backend.add_blog(formData.title, formData.content, formData.tags.split(","));

        if ('Ok' in response) {
            console.log("Blog added:", response.Ok);
            formData.title = "";
            formData.content = "";
            formData.tags = "";
        } else {
            console.error("Error:", response.Err);
        }
    };

    (async function getBlog() {
        try {
            const response = await blog_backend.get_blogs();
            blogs.value = response.map((blog) => {
                return {
                    ...blog,
                    date: new Date(Number(blog.date / 1000000n)).toLocaleString()
                }
            });
        } catch (error) {
            console.error("Error fetching blogs:", error);
        }
    })();


</script>