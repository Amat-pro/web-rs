<template>
  <div class="common-layout">
    <el-container>
      <el-header></el-header>
      <el-main>
        <el-row class="mb-4">
          <el-button type="success" round>登录</el-button>
          <el-button type="success" round>注册</el-button>
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
  mounted() {
    console.log("todo check token");
    let token_valid = true;
    if (token_valid) {
      this.$router.replace("/blog-home");
    }
  },
});
</script>
