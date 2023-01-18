<template>
  <div class="blog-home">
    <div class="buttons">
      <el-button type="success" round @click="login">登录</el-button>
      <el-button type="success" round @click="register">注册</el-button>
      <el-button type="success" round @click="viewArticle">游客</el-button>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
export default defineComponent({
  name: "BlogHome",
  methods: {
    login() {
      this.$router.push("/login");
    },
    register() {
      this.$router.push("/register");
    },
    viewArticle() {
      this.$router.push("/article");
    },
  },
  mounted() {
    console.log("=== mounted");
  },
});
</script>

<style>
.blog-home {
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 0;
  display: flex;
}

.buttons {
  margin: auto auto;
}
</style>
