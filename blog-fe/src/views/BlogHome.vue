<template>
  <div class="blog-home">
    <el-container>
      <el-header></el-header>
      <el-main>
        <el-row class="login-register">
          <el-button type="success" round @click="login">登录</el-button>
          <el-button type="success" round @click="register">注册</el-button>
          <el-button type="success" round @click="viewArticle">游客</el-button>
        </el-row>
      </el-main>
      <el-footer></el-footer>
    </el-container>
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
    console.log("todo check token");
    let token_valid = true;
    if (token_valid) {
      this.$router.replace("/blog-home");
    }
  },
});
</script>
