<template>
    <main>
        <h1>ICP Bootcamp Blog</h1>
        <div>
            <form action="#" @submit="handleSubmit">
                <h2>Add new blog</h2>
                <div>
                    <label for="title">Title: </label>
                    <input id="title" type="text" v-model="formData.title"/>
                </div>
                <div>
                    <label for="content">Content: </label>
                    <input id="content" type="text" v-model="formData.content"/>
                </div>
                <div>
                    <label for="tags">Tags: </label>
                    <input id="tags" type="text" v-model="formData.tags"/>
                </div>
                <button type="submit">Add Blog</button>
            </form>
        </div>

        <div class="blog">
            <h2>Our blogs:</h2>
            <div class="blog__entry" v-for="(blog, key) in blogs" :key="key">
                <p>Title: {{ blog.title }}</p>
                <p>Content: {{ blog.content }}</p>
                <p>Tags: {{ blog.tags.toString() }}</p>
                <p>Date: {{ blog.date }}</p>
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

<style lang="css">
    main {
        width: 100%;
        max-width: 1440px;
        margin: 0 auto;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 15px;
        padding: 20px 0px;
    }

    form {
        display: flex;
        flex-direction: column;
        border: 1px solid black;
        padding: 20px;
        gap: 15px;
        width: 700px;
    }

    input {
        width: 100%;
    }

    button[type=submit] {
        padding: 5px 10px;
    }

    .blog {
       display: flex;
       flex-direction: column;
       width: 700px;
       gap: 15px;
    }

    .blog__entry {
        border: 1px solid black;
        padding: 10px 25px;
    }
</style>